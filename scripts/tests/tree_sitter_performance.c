// Native editor workload: chunked input, pooled cursors, and shared edit snapshots.
#include <tree_sitter/api.h>
#include <assert.h>
#include <pthread.h>
#include <stdatomic.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

extern const TSLanguage *tree_sitter_json(void);
typedef union { size_t size; max_align_t alignment; } Allocation;
static _Atomic size_t allocations, live_bytes, peak_bytes;

static void account(size_t size) {
  size_t live = atomic_fetch_add(&live_bytes, size) + size;
  size_t peak = atomic_load(&peak_bytes);
  while (live > peak && !atomic_compare_exchange_weak(&peak_bytes, &peak, live)) {}
}
static void *allocate(size_t size) {
  Allocation *p = malloc(sizeof(Allocation) + size);
  assert(p); p->size = size; account(size); atomic_fetch_add(&allocations, 1);
  return p + 1;
}
static void deallocate(void *p) {
  if (!p) return;
  Allocation *a = (Allocation *)p - 1;
  atomic_fetch_sub(&live_bytes, a->size); free(a);
}
static void *resize(void *p, size_t size) {
  if (!p) return allocate(size);
  Allocation *a = (Allocation *)p - 1;
  size_t old = a->size;
  a = realloc(a, sizeof(Allocation) + size); assert(a);
  a->size = size;
  atomic_fetch_sub(&live_bytes, old); account(size);
  atomic_fetch_add(&allocations, 1);
  return a + 1;
}
static void *zero_allocate(size_t n, size_t size) {
  void *p = allocate(n * size); memset(p, 0, n * size); return p;
}
static double now_ms(void) {
  struct timespec t; clock_gettime(CLOCK_MONOTONIC, &t);
  return t.tv_sec * 1000.0 + t.tv_nsec / 1e6;
}
typedef struct { const char *text; uint32_t len, chunk; } Input;
static const char *read_input(void *payload, uint32_t offset, TSPoint point, uint32_t *length) {
  (void)point; Input *input = payload;
  if (offset >= input->len) { *length = 0; return ""; }
  *length = input->len - offset;
  if (*length > input->chunk) *length = input->chunk;
  return input->text + offset;
}
static TSTree *parse(TSParser *parser, const TSTree *old, Input *input) {
  return ts_parser_parse(parser, old, (TSInput){input, read_input, TSInputEncodingUTF8, NULL});
}
static char *load(const char *path, uint32_t *len) {
  FILE *f = fopen(path, "rb"); assert(f);
  fseek(f, 0, SEEK_END); long size = ftell(f); assert(size >= 0);
  rewind(f); char *s = malloc((size_t)size + 1); assert(s);
  assert(fread(s, 1, (size_t)size, f) == (size_t)size); fclose(f);
  s[size] = 0; *len = (uint32_t)size; return s;
}
static uint64_t captures(TSQueryCursor *cursor, TSQuery *query, TSTree *tree, bool dump) {
  ts_query_cursor_exec(cursor, query, ts_tree_root_node(tree));
  TSQueryMatch match; uint32_t index; uint64_t count = 0;
  while (ts_query_cursor_next_capture(cursor, &match, &index)) {
    TSQueryCapture capture = match.captures[index];
    if (dump) printf("capture %u %u %u %u\n", match.pattern_index, capture.index,
      ts_node_start_byte(capture.node), ts_node_end_byte(capture.node));
    count++;
  }
  if (dump) printf("exceeded %d\n", ts_query_cursor_did_exceed_match_limit(cursor));
  return count;
}
static void *share_tree(void *payload) {
  const TSTree *tree = payload;
  for (unsigned i = 0; i < 200; i++) {
    TSTree *copy = ts_tree_copy(tree);
    assert(ts_node_child_count(ts_tree_root_node(copy)) > 0);
    TSInputEdit edit = {1, 1, 2, {0, 1}, {0, 1}, {0, 2}};
    ts_tree_edit(copy, &edit);
    ts_tree_delete(copy);
  }
  return NULL;
}
static bool cancel(TSParseState *state) { (*(unsigned *)state->payload)++; return true; }
static bool keep_parsing(TSParseState *state) { (*(unsigned *)state->payload)++; return false; }

static void query_regressions(TSParser *parser, TSQueryCursor *cursor, const TSLanguage *language, bool dump) {
  const char *sources[] = {
    "", "{}", "[1,2,3]", "[1,\"x\",2]", "{\"a\":1,\"b\":{\"c\":[1,2]}}",
    "{\"one\":{\"bar\" \"baz\"},\"two\":\"bar\"}", "[1,,2", "{\"a\":}",
    "[1, /* comment */ 2, null]", "{\"a\":\"\\u1234\",\"b\":\"日本語\"}\r\n",
    "[true,false,null,-12.34e+56]", "\"unterminated\\", "[[[[[[[[0]]]]]]]]",
  };
  const char *queries[] = {
    "(_) @all", "(ERROR) @error", "(string) @s (number) @n (true) @b",
    "(pair key: (string) @key value: (_) @value) @pair",
    "(array . (number)* @numbers .) @array", "(array (number) @a . (number) @b)",
    "(object (pair key: (string) @key)+) @object", "(_ (string) @string) @parent",
    "(array [(number) (string)]* @values)", "(object !key) @object",
  };
  for (unsigned q = 0; q < sizeof(queries) / sizeof(*queries); q++) {
    uint32_t offset; TSQueryError error;
    TSQuery *query = ts_query_new(language, queries[q], (uint32_t)strlen(queries[q]), &offset, &error);
    assert(query);
    for (unsigned s = 0; s < sizeof(sources) / sizeof(*sources); s++) {
      Input input = {sources[s], (uint32_t)strlen(sources[s]), 3};
      TSTree *tree = parse(parser, NULL, &input); assert(tree);
      for (uint32_t start = 0; start <= input.len; start += 3) {
        uint32_t end = start + 7;
        ts_query_cursor_set_byte_range(cursor, start, end);
        if (dump) printf("fixture %u %u %u\n", q, s, start);
        captures(cursor, query, tree, dump);
      }
      ts_tree_delete(tree);
    }
    ts_query_delete(query);
  }
}

int main(int argc, char **argv) {
  assert(argc == 3);
  ts_set_allocator(allocate, zero_allocate, resize, deallocate);
  bool dump = strcmp(argv[2], "captures") == 0;
  uint32_t query_len, error_offset; TSQueryError error_type;
  char *query_text = load(argv[1], &query_len);
  const TSLanguage *language = tree_sitter_json();
  TSQuery *query = ts_query_new(language, query_text, query_len, &error_offset, &error_type);
  assert(query); free(query_text);
  TSParser *parser = ts_parser_new(); assert(ts_parser_set_language(parser, language));
  TSQueryCursor *cursor = ts_query_cursor_new(); ts_query_cursor_set_match_limit(cursor, 64);
  const char *row = "{\"key\":123,\"text\":\"hello\\nworld\",\"unicode\":\"日本語\",\"items\":[true,false,null]},\n";
  unsigned rows = 4000; size_t row_len = strlen(row);
  char *source = malloc(rows * row_len + 4); assert(source);
  source[0] = '['; source[1] = '\n';
  for (unsigned i = 0; i < rows; i++) memcpy(source + 2 + i * row_len, row, row_len);
  size_t len = 2 + rows * row_len;
  source[len - 2] = '\n'; source[len++] = ']'; source[len] = 0;
  Input input = {source, (uint32_t)len, 1024};
  double start = now_ms();
  TSTree *tree = parse(parser, NULL, &input); assert(tree);
  assert(!ts_node_has_error(ts_tree_root_node(tree)));
  if (!dump) printf("parse_ms %.4f\n", now_ms() - start);
  unsigned progress_calls = 0;
  start = now_ms();
  TSTree *with_callback = ts_parser_parse_with_options(parser, NULL,
    (TSInput){&input, read_input, TSInputEncodingUTF8, NULL},
    (TSParseOptions){&progress_calls, keep_parsing});
  assert(with_callback && !ts_node_has_error(ts_tree_root_node(with_callback)));
  if (!dump) printf("callback_parse_ms %.4f\n", now_ms() - start);
  ts_tree_delete(with_callback);
  size_t alloc_before = atomic_load(&allocations);
  start = now_ms(); uint64_t count = 0;
  for (unsigned i = 0; i < 100; i++) {
    uint32_t offset = i * (input.len - 4096) / 100;
    ts_query_cursor_set_byte_range(cursor, offset, offset + 4096);
    if (dump) printf("window %u\n", i);
    count += captures(cursor, query, tree, dump);
  }
  if (!dump) printf("query_ms %.4f\nquery_allocations %zu\ncaptures %llu\n",
    now_ms() - start, atomic_load(&allocations) - alloc_before, (unsigned long long)count);
  start = now_ms(); alloc_before = atomic_load(&allocations);
  for (unsigned i = 0; i < 100; i++) {
    TSTree *snapshot = ts_tree_copy(tree);
    uint32_t offset = 9 + (i % rows) * (uint32_t)row_len;
    TSInputEdit edit = {offset, offset + 1, offset + 1, {i % rows + 1, 7}, {i % rows + 1, 8}, {i % rows + 1, 8}};
    source[offset] = source[offset] == '1' ? '2' : '1';
    ts_tree_edit(tree, &edit);
    TSTree *next = parse(parser, tree, &input); assert(next);
    assert(!ts_node_has_error(ts_tree_root_node(next)));
    uint32_t range_count; TSRange *ranges = ts_tree_get_changed_ranges(tree, next, &range_count);
    deallocate(ranges); ts_tree_delete(tree); ts_tree_delete(snapshot); tree = next;
  }
  if (!dump) printf("edit_ms %.4f\nedit_allocations %zu\n", now_ms() - start, atomic_load(&allocations) - alloc_before);
  start = now_ms();
  for (unsigned i = 0; i < 100000; i++) ts_tree_delete(ts_tree_copy(tree));
  if (!dump) printf("copy_drop_ms %.4f\n", now_ms() - start);
  pthread_t threads[4];
  for (unsigned i = 0; i < 4; i++) assert(pthread_create(&threads[i], NULL, share_tree, tree) == 0);
  for (unsigned i = 0; i < 4; i++) assert(pthread_join(threads[i], NULL) == 0);
  TSTree *wide_edit = ts_tree_copy(tree);
  TSPoint end_point = ts_node_end_point(ts_tree_root_node(wide_edit));
  TSInputEdit deletion = {0, input.len, 0, {0, 0}, end_point, {0, 0}};
  ts_tree_edit(wide_edit, &deletion);
  ts_tree_delete(wide_edit);
  ts_tree_delete(tree); free(source);
  query_regressions(parser, cursor, language, dump);

  unsigned size = 16 * 1024 * 1024, callbacks = 0;
  char *string = malloc(size + 2); assert(string);
  string[0] = '"'; memset(string + 1, 'a', size); string[size + 1] = '"';
  Input long_input = {string, size + 2, 1024};
  TSInput ts_input = {&long_input, read_input, TSInputEncodingUTF8, NULL};
  start = now_ms();
  tree = ts_parser_parse_with_options(parser, NULL, ts_input, (TSParseOptions){&callbacks, cancel});
  if (!dump) printf("cancel_ms %.4f\ncancel_callbacks %u\n", now_ms() - start, callbacks);
  if (tree) ts_tree_delete(tree);
  // A cancelled token must never be cached as an EOF or a completed token.
  tree = parse(parser, NULL, &long_input); assert(tree);
  assert(!ts_node_has_error(ts_tree_root_node(tree)));
  assert(ts_node_end_byte(ts_tree_root_node(tree)) == size + 2);
  ts_tree_delete(tree); free(string);
  ts_query_cursor_delete(cursor); ts_query_delete(query); ts_parser_delete(parser);
  if (!dump) printf("peak_bytes %zu\nremaining_bytes %zu\n", atomic_load(&peak_bytes), atomic_load(&live_bytes));
  assert(atomic_load(&live_bytes) == 0);
}
