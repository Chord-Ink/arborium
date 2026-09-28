// Internal pool invariants and an isolated benchmark for tree-sitter#5951.
// Compile with the selected runtime's src/ and include/ on the include path.
#include <assert.h>
#include <stdio.h>
#include <time.h>
#include "lib.c"

static void check_pool(void) {
  CaptureListPool pool = capture_list_pool_new();
  pool.max_capture_list_count = 8;
  for (uint32_t i = 0; i < 8; i++) {
    uint32_t id = capture_list_pool_acquire(&pool);
    assert(id == i);
    TSQueryCapture capture = {.index = i};
    array_push(capture_list_pool_get_mut(&pool, id), capture);
  }
  assert(capture_list_pool_is_empty(&pool));
  assert(capture_list_pool_acquire(&pool) == CAPTURE_LIST_NONE);
  capture_list_pool_release(&pool, 3);
  capture_list_pool_release(&pool, 7);
  capture_list_pool_release(&pool, CAPTURE_LIST_NONE);
  uint32_t first = capture_list_pool_acquire(&pool);
  uint32_t second = capture_list_pool_acquire(&pool);
  assert((first == 3 && second == 7) || (first == 7 && second == 3));
  assert(capture_list_pool_get(&pool, first)->size == 0);
  assert(capture_list_pool_get(&pool, second)->size == 0);
  assert(capture_list_pool_is_empty(&pool));

  for (unsigned repeat = 0; repeat < 3; repeat++) {
    capture_list_pool_reset(&pool);
    bool seen[8] = {false};
    for (unsigned i = 0; i < 8; i++) {
      uint32_t id = capture_list_pool_acquire(&pool);
      assert(id < 8 && !seen[id]);
      assert(capture_list_pool_get(&pool, id)->size == 0);
      seen[id] = true;
    }
    assert(capture_list_pool_acquire(&pool) == CAPTURE_LIST_NONE);
  }
  capture_list_pool_delete(&pool);
}

int main(void) {
  check_pool();
  const uint32_t sizes[] = {4096, 32768};
  for (unsigned s = 0; s < 2; s++) {
    CaptureListPool pool = capture_list_pool_new();
    for (uint32_t i = 0; i < sizes[s]; i++) {
      assert(capture_list_pool_acquire(&pool) == i);
    }
    clock_t start = clock();
    uint64_t checksum = 0;
    for (unsigned iteration = 0; iteration < 20000; iteration++) {
      capture_list_pool_release(&pool, sizes[s] - 1);
      checksum += capture_list_pool_acquire(&pool);
    }
    assert(checksum == (uint64_t)(sizes[s] - 1) * 20000);
    printf("%u held lists, 20000 reuses: %.3f ms (checksum %llu)\n",
      sizes[s], 1000.0 * (clock() - start) / CLOCKS_PER_SEC,
      (unsigned long long)checksum);
    capture_list_pool_delete(&pool);
  }
  return 0;
}
