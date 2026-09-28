// Compare complete query output across runtimes, including malformed prefixes.
#include <tree_sitter/api.h>
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

extern const TSLanguage *LANGUAGE(void);
static char *load(const char *path, uint32_t *length) {
  FILE *file = fopen(path, "rb"); assert(file);
  fseek(file, 0, SEEK_END); long size = ftell(file); assert(size >= 0);
  rewind(file); char *text = malloc(size + 1); assert(text);
  assert(fread(text, 1, size, file) == (size_t)size); fclose(file);
  text[size] = 0; *length = size; return text;
}
static bool cancel(TSParseState *state) { (*(unsigned *)state->payload)++; return true; }
typedef struct { const char *source; uint32_t length; } Input;
static const char *read_input(void *payload, uint32_t byte, TSPoint point, uint32_t *length) {
  (void)point; Input *input = payload;
  *length = byte < input->length ? input->length - byte : 0;
  if (*length > 1024) *length = 1024;
  return *length ? input->source + byte : "";
}

int main(int argc, char **argv) {
  assert(argc >= 3);
  uint32_t length; char *source = load(argv[1], &length);
  TSParser *parser = ts_parser_new();
  assert(ts_parser_set_language(parser, LANGUAGE()));
  if (!strcmp(argv[2], "cancel")) {
    // Check generated lexers, whitespace, UTF-16, and external scanners.
    for (unsigned encoding = 0; encoding < 2; encoding++) {
      uint16_t *utf16 = malloc(length * sizeof(uint16_t)); assert(utf16);
      for (unsigned i = 0; i < length; i++) utf16[i] = (unsigned char)source[i];
      Input input = {encoding ? (char *)utf16 : source, length * (encoding ? 2 : 1)};
      TSInput ts_input = {&input, read_input, encoding ? TSInputEncodingUTF16LE : TSInputEncodingUTF8, NULL};
      unsigned calls = 0;
      TSTree *cancelled = ts_parser_parse_with_options(parser, NULL, ts_input, (TSParseOptions){&calls, cancel});
      assert(!cancelled && calls == 1);
      TSTree *resumed = ts_parser_parse(parser, NULL, ts_input); assert(resumed);
      ts_parser_reset(parser);
      TSTree *fresh = ts_parser_parse(parser, NULL, ts_input); assert(fresh);
      char *a = ts_node_string(ts_tree_root_node(resumed));
      char *b = ts_node_string(ts_tree_root_node(fresh));
      assert(!strcmp(a, b));
      assert(ts_node_end_byte(ts_tree_root_node(resumed)) == ts_node_end_byte(ts_tree_root_node(fresh)));
      free(a); free(b); ts_tree_delete(resumed); ts_tree_delete(fresh);
      calls = 0;
      assert(!ts_parser_parse_with_options(parser, NULL, ts_input, (TSParseOptions){&calls, cancel}));
      ts_parser_reset(parser);
      TSTree *empty = ts_parser_parse_string(parser, NULL, "", 0); assert(empty);
      assert(ts_node_end_byte(ts_tree_root_node(empty)) == 0);
      ts_tree_delete(empty); free(utf16);
    }
  } else {
    for (int q = 2; q < argc; q++) {
      uint32_t query_length, offset; TSQueryError error;
      char *query_text = load(argv[q], &query_length);
      TSQuery *query = ts_query_new(LANGUAGE(), query_text, query_length, &offset, &error);
      if (!query) { fprintf(stderr, "%s: query error %d at %u\n", argv[q], error, offset); return 1; }
      free(query_text);
      TSQueryCursor *cursor = ts_query_cursor_new();
      ts_query_cursor_set_match_limit(cursor, 64);
      for (unsigned cut = 0; cut < 8; cut++) {
        uint32_t end = length * (8 - cut) / 8;
        TSTree *tree = ts_parser_parse_string(parser, NULL, source, end); assert(tree);
        char *sexp = ts_node_string(ts_tree_root_node(tree)); puts(sexp); free(sexp);
        for (unsigned window = 0; window < 5; window++) {
          uint32_t start = window ? end * (window - 1) / 4 : 0;
          ts_query_cursor_set_byte_range(cursor, start, window ? start + 96 : end);
          for (unsigned mode = 0; mode < 2; mode++) {
            printf("query %d cut %u window %u mode %u\n", q, cut, window, mode);
            ts_query_cursor_exec(cursor, query, ts_tree_root_node(tree));
            TSQueryMatch match; uint32_t index;
            while (mode ? ts_query_cursor_next_match(cursor, &match) : ts_query_cursor_next_capture(cursor, &match, &index)) {
              uint32_t first = mode ? 0 : index, last = mode ? match.capture_count : index + 1;
              printf("match %u\n", match.pattern_index);
              for (uint32_t c = first; c < last; c++) {
                TSQueryCapture capture = match.captures[c];
                printf("%u %u %u\n", capture.index, ts_node_start_byte(capture.node), ts_node_end_byte(capture.node));
              }
            }
            printf("exceeded %d\n", ts_query_cursor_did_exceed_match_limit(cursor));
          }
        }
        ts_tree_delete(tree);
      }
      ts_query_cursor_delete(cursor); ts_query_delete(query);
    }
  }
  ts_parser_delete(parser); free(source);
}
