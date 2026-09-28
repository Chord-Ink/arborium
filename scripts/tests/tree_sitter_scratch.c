// Internal retention assertions complement public capture-output comparisons.
#include <assert.h>
#include <stdio.h>
#include "lib.c"

static size_t retained_bytes(CaptureListPool *pool) {
  size_t result = pool->list.capacity * sizeof(CaptureList) + pool->free_ids.capacity * sizeof(uint32_t);
  for (unsigned i = 0; i < pool->list.size; i++) result += array_get(&pool->list, i)->capacity * sizeof(TSQueryCapture);
  return result;
}

int main(void) {
  CaptureListPool pool = capture_list_pool_new();
  for (unsigned i = 0; i < 4096; i++) {
    unsigned id = capture_list_pool_acquire(&pool);
    if (i == 0) {
      CaptureList *list = capture_list_pool_get_mut(&pool, id);
      array_reserve(list, 16384);
      list->size = 16384;
    }
  }
  capture_list_pool_reset(&pool);
  // One small query must not discard the preceding working set.
  unsigned id = capture_list_pool_acquire(&pool);
  capture_list_pool_release(&pool, id);
  capture_list_pool_reset(&pool);
  assert(pool.list.size == 4096);
  size_t before = retained_bytes(&pool);
  for (unsigned i = 0; i < 8; i++) {
    id = capture_list_pool_acquire(&pool);
    capture_list_pool_release(&pool, id);
    capture_list_pool_reset(&pool);
  }
  assert(pool.list.size <= 64);
  for (unsigned i = 0; i < pool.list.size; i++) {
    assert(array_get(&pool.list, i)->capacity <= 1024);
  }
  printf("Capture scratch retained bytes: %zu -> %zu after low-use resets\n", before, retained_bytes(&pool));
  // Reuse and regrow after trimming without duplicating IDs.
  bool seen[128] = {false};
  for (unsigned i = 0; i < 128; i++) {
    id = capture_list_pool_acquire(&pool);
    assert(id < 128 && !seen[id]); seen[id] = true;
  }
  capture_list_pool_delete(&pool);
}
