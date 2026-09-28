// Internal regressions for #5807, #5949 and #5910. No generated grammar needed.
// Use a small native thread stack and keep assertions enabled.
#include <assert.h>
#include <pthread.h>
#include <stdio.h>
#include "lib.c"

static const TSSymbolMetadata metadata[] = {
  {false, false, false}, {true, true, false},
  {false, false, false}, {true, true, false},
};
static const char *const names[] = {"end", "node", "hidden", "leaf"};
static const TSFieldMapEntry fields[] = {
  {1, 0, true}, {1, 1, true}, {1, 2, true},
  {2, 0, true}, {2, 1, true}, {2, 2, true},
  {1, 0, false},
};
static const TSMapSlice field_slices[] = {{0, 0}, {0, 6}, {6, 1}};
static const char *const field_names[] = {NULL, "found", "absent"};
static const TSLanguage language = {
  .abi_version = TREE_SITTER_LANGUAGE_VERSION,
  .symbol_count = 4,
  .symbol_metadata = metadata,
  .symbol_names = names,
  .field_count = 2,
  .field_names = field_names,
  .field_map_slices = field_slices,
  .field_map_entries = fields,
};

static Subtree leaf(SubtreePool *pool, uint32_t bytes, uint32_t lookahead) {
  return ts_subtree_new_leaf(pool, 3, length_zero(), (Length){bytes, {0, bytes}},
    lookahead, 0, false, false, false, &language);
}

// Takes ownership of its children, just like a parser reduction.
static Subtree node(TSSymbol symbol, unsigned production, Subtree *children, unsigned count) {
  SubtreeArray array = array_new();
  array_extend(&array, count, children);
  return ts_subtree_from_mut(ts_subtree_new_node(symbol, &array, production, &language));
}

static void check_traversal(void) {
  SubtreePool pool = ts_subtree_pool_new(0);
  // A heap leaf has a unique identity even though its range is empty.
  Subtree target = leaf(&pool, 0, 512);
  Subtree absent = leaf(&pool, 0, 513);
  Subtree root = node(2, 2, &target, 1);
  const Subtree *target_id = ts_subtree_children(root);
  for (unsigned i = 0; i < 20000; i++) {
    Subtree children[] = {leaf(&pool, 0, 0), root, leaf(&pool, 0, 0)};
    root = node(2, 1, children, 3);
  }
  root = node(1, 1, &root, 1);
  TSRange range = {0};
  TSTree *tree = ts_tree_new(root, &language, &range, 1);
  TSNode parent = ts_tree_root_node(tree);
  TSNode descendant = ts_node_new(tree, target_id, length_zero(), 0);
  assert(ts_node_eq(ts_node_child_by_field_id(parent, 1), descendant));
  assert(ts_node_is_null(ts_node_child_by_field_id(parent, 2)));
  assert(ts_subtree_has_trailing_empty_descendant(root, target));
  assert(!ts_subtree_has_trailing_empty_descendant(root, absent));
  assert(ts_node_eq(ts_node_child_with_descendant(parent, descendant), descendant));
  // A later empty sibling requires resuming the search after the deep branch.
  TSNode last = ts_node_child(parent, ts_node_child_count(parent) - 1);
  assert(ts_node_eq(ts_node_child_with_descendant(parent, last), last));
  assert(ts_node_is_null(ts_node_child_with_descendant(descendant, parent)));
  assert(ts_node_is_null(ts_node_child_with_descendant(parent, parent)));
  TSTree *copy = ts_tree_copy(tree);
  assert(ts_node_is_null(ts_node_child_with_descendant(ts_tree_root_node(copy), descendant)));
  ts_tree_delete(copy);
  FILE *file = fopen("/dev/null", "w");
  assert(file);
  ts_subtree_print_dot_graph(root, &language, file);
  fclose(file);
  ts_tree_delete(tree);
  ts_subtree_release(&pool, absent);

  // Visible, equal-range ancestors: identity, not the range, decides ancestry.
  root = node(1, 0, (Subtree[]){leaf(&pool, 0, 0)}, 1);
  target_id = ts_subtree_children(root);
  for (unsigned i = 0; i < 20000; i++) {
    root = node(1, 0, (Subtree[]){root, leaf(&pool, 0, 0)}, 2);
  }
  tree = ts_tree_new(root, &language, &range, 1);
  parent = ts_tree_root_node(tree);
  descendant = ts_node_new(tree, target_id, length_zero(), 0);
  assert(ts_node_eq(ts_node_child_with_descendant(parent, descendant), ts_node_child(parent, 0)));
  assert(ts_node_is_null(ts_node_child_with_descendant(ts_node_child(parent, 0), parent)));
  ts_tree_delete(tree);
  ts_subtree_pool_delete(&pool);
}

static void check_error_cost(void) {
  SubtreePool pool = ts_subtree_pool_new(0);
  // Hidden recovery grouping must not change the cost, including lexical errors.
  for (unsigned lexical = 0; lexical < 2; lexical++) {
    Subtree tokens[] = {
      leaf(&pool, 1, 0),
      lexical ? ts_subtree_new_error(&pool, '?', length_zero(), (Length){1, {0, 1}}, 1, 0, &language)
              : leaf(&pool, 1, 0),
      leaf(&pool, 1, 0),
    };
    for (unsigned i = 0; i < 3; i++) ts_subtree_retain(tokens[i]);
    Subtree flat = node(ts_builtin_sym_error, 0, tokens, 3);
    Subtree hidden = node(ts_builtin_sym_error_repeat, 0, tokens, 2);
    // Multiple wrapper levels must also remain cost-neutral.
    hidden = node(ts_builtin_sym_error_repeat, 0, &hidden, 1);
    Subtree grouped = node(ts_builtin_sym_error, 0, (Subtree[]){hidden, tokens[2]}, 2);
    assert(ts_subtree_error_cost(flat) == ts_subtree_error_cost(grouped));
    assert(ts_subtree_error_cost(flat) == ERROR_COST_PER_RECOVERY + 3 * ERROR_COST_PER_SKIPPED_TREE + 3 * ERROR_COST_PER_SKIPPED_CHAR);
    ts_subtree_release(&pool, flat);
    ts_subtree_release(&pool, grouped);
  }
  ts_subtree_pool_delete(&pool);
}

static void check_utf16(void) {
  // Upstream #5912: the trailing surrogate must use the input endianness too.
  const uint16_t pairs[][2] = {{0xd800, 0xdc00}, {0xd83d, 0xde00}, {0xdbff, 0xdfff}};
  const int32_t expected[] = {0x10000, 0x1f600, 0x10ffff};
  for (unsigned i = 0; i < 3; i++) {
    const uint16_t le[] = {htole16(pairs[i][0]), htole16(pairs[i][1])};
    const uint16_t be[] = {htobe16(pairs[i][0]), htobe16(pairs[i][1])};
    int32_t codepoint;
    assert(ts_decode_utf16_le((const uint8_t *)le, sizeof(le), &codepoint) == 4);
    assert(codepoint == expected[i]);
    assert(ts_decode_utf16_be((const uint8_t *)be, sizeof(be), &codepoint) == 4);
    assert(codepoint == expected[i]);
  }
}

static void *check(void *unused) {
  (void)unused;
  check_traversal();
  check_error_cost();
  check_utf16();
  return NULL;
}

int main(void) {
  pthread_attr_t attr;
  pthread_t thread;
  assert(pthread_attr_init(&attr) == 0);
  assert(pthread_attr_setstacksize(&attr, 256 * 1024) == 0);
  assert(pthread_create(&thread, &attr, check, NULL) == 0);
  assert(pthread_join(thread, NULL) == 0);
  assert(pthread_attr_destroy(&attr) == 0);
  puts("Deep traversal, UTF-16 and recovery-cost invariants passed.");
  return 0;
}
