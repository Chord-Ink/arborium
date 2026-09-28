#ifndef TREE_SITTER_ATOMIC_H_
#define TREE_SITTER_ATOMIC_H_

#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __TINYC__

static inline size_t atomic_load(const volatile size_t *p) {
  return *p;
}

static inline uint32_t atomic_inc(volatile uint32_t *p) {
  *p += 1;
  return *p;
}

static inline uint32_t atomic_dec(volatile uint32_t *p) {
  *p-= 1;
  return *p;
}

#elif defined(_WIN32)

#include <windows.h>

static inline size_t atomic_load(const volatile size_t *p) {
  return *p;
}

static inline uint32_t atomic_inc(volatile uint32_t *p) {
  return InterlockedIncrement((long volatile *)p);
}

static inline uint32_t atomic_dec(volatile uint32_t *p) {
  return InterlockedDecrement((long volatile *)p);
}

#else

static inline size_t atomic_load(const volatile size_t *p) {
#ifdef __ATOMIC_RELAXED
  return __atomic_load_n(p, __ATOMIC_RELAXED);
#else
  return __sync_fetch_and_add((volatile size_t *)p, 0);
#endif
}

static inline uint32_t atomic_inc(volatile uint32_t *p) {
  #ifdef __ATOMIC_RELAXED
    return __atomic_add_fetch(p, 1U, __ATOMIC_SEQ_CST);
  #else
    return __sync_add_and_fetch(p, 1U);
  #endif
}

static inline uint32_t atomic_dec(volatile uint32_t *p) {
  #ifdef __ATOMIC_RELAXED
    return __atomic_sub_fetch(p, 1U, __ATOMIC_SEQ_CST);
  #else
    return __sync_sub_and_fetch(p, 1U);
  #endif
}

#endif

// These operations apply only to owning subtree references. Other atomic
// counters (including WASM deletion notifications) retain their original ordering.
static inline uint32_t atomic_ref_count_load(const volatile uint32_t *p) {
#if defined(__ATOMIC_ACQUIRE) && !defined(_WIN32) && !defined(__TINYC__)
  return __atomic_load_n(p, __ATOMIC_ACQUIRE);
#elif defined(_WIN32) && !defined(__TINYC__)
  return (uint32_t)InterlockedCompareExchange((long volatile *)p, 0, 0);
#elif !defined(__TINYC__)
  return __sync_fetch_and_add((volatile uint32_t *)p, 0);
#else
  return *p;
#endif
}

static inline void atomic_retain(volatile uint32_t *p) {
#if defined(__ATOMIC_RELAXED) && !defined(_WIN32) && !defined(__TINYC__)
  uint32_t old = __atomic_fetch_add(p, 1U, __ATOMIC_RELAXED);
  if (old >= UINT32_MAX / 2) abort();
#else
  if (atomic_inc(p) > UINT32_MAX / 2) abort();
#endif
}

static inline uint32_t atomic_release(volatile uint32_t *p) {
#if defined(__ATOMIC_RELEASE) && !defined(_WIN32) && !defined(__TINYC__)
  uint32_t count = __atomic_sub_fetch(p, 1U, __ATOMIC_RELEASE);
  // Acquire the release sequence before destruction. Use a load rather than
  // a fence so race detectors can observe the synchronization too.
  if (count == 0) (void)__atomic_load_n(p, __ATOMIC_ACQUIRE);
  return count;
#else
  return atomic_dec(p);
#endif
}

#endif  // TREE_SITTER_ATOMIC_H_
