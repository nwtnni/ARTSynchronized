#include "wrap.h"
#include <iostream>

typedef struct pair_u64 {
  uint64_t key;
  uint64_t value;
} pair_u64_t;

static void load_u64(TID tid, Key &key) {
  key.setKeyLen(sizeof(uint64_t));
  reinterpret_cast<uint64_t *>(&key[0])[0] =
      __builtin_bswap64(reinterpret_cast<pair_u64 *>(tid)->key);
}

std::unique_ptr<EpochInfo> rowex_info(Rowex *rowex) {
  return std::make_unique<EpochInfo>(rowex->getThreadInfo());
}

std::unique_ptr<Rowex> rowex_u64_new() {
  return std::make_unique<Rowex>(load_u64);
}

bool rowex_u64_insert(Rowex *rowex, uint64_t key, uint64_t value,
                      EpochInfo *epocheInfo) {
  auto pair = new pair_u64_t{key, value};
  auto tid = reinterpret_cast<uint64_t>(pair);
  Key byte_key;
  load_u64(tid, byte_key);
  if (rowex->insert(byte_key, tid, *epocheInfo)) {
    return true;
  } else {
    delete pair;
    return false;
  }
}

bool rowex_u64_lookup(Rowex *rowex, uint64_t key, uint64_t *value,
                      EpochInfo *epocheInfo) {
  Key byte_key;
  byte_key.setKeyLen(sizeof(uint64_t));
  reinterpret_cast<uint64_t *>(&byte_key[0])[0] = __builtin_bswap64(key);
  auto tid = rowex->lookup(byte_key, *epocheInfo);
  if (tid == 0) {
    return false;
  }
  *value = reinterpret_cast<pair_u64_t *>(tid)->value;
  return true;
}

bool rowex_u64_lookup_range(Rowex *rowex, uint64_t start, uint64_t end,
                            uint64_t result[], size_t resultSize,
                            size_t *resultsFound, EpochInfo *epocheInfo) {
  Key start_key;
  start_key.setKeyLen(sizeof(uint64_t));
  reinterpret_cast<uint64_t *>(&start_key[0])[0] = __builtin_bswap64(start);

  Key end_key;
  end_key.setKeyLen(sizeof(uint64_t));
  reinterpret_cast<uint64_t *>(&end_key[0])[0] = __builtin_bswap64(end);

  Key continue_key;

  // NOTE: returns TIDs instead of values, fine for conservative performance
  // estimate but not for real use
  return rowex->lookupRange(start_key, end_key, continue_key, result,
                            resultSize, *resultsFound, *epocheInfo);
}

typedef struct pair_string {
  Key key;
  uint64_t value;
} pair_string_t;

static void load_string(TID tid, Key &key) {
  auto pair = reinterpret_cast<pair_string_t *>(tid);
  key = pair->key;
}

std::unique_ptr<Rowex> rowex_string_new() {
  return std::make_unique<Rowex>(load_string);
}

bool rowex_string_insert(Rowex *rowex, const char *kbuf, size_t klen,
                         uint64_t value, EpochInfo *epocheInfo) {
  auto pair = new pair_string_t;
  pair->key.set(kbuf, klen);
  pair->value = value;
  auto tid = reinterpret_cast<uint64_t>(pair);
  if (rowex->insert(pair->key, tid, *epocheInfo)) {
    return true;
  } else {
    delete pair;
    return false;
  }
}

bool rowex_string_lookup(Rowex *rowex, const char *kbuf, size_t klen,
                         uint64_t *value, EpochInfo *epocheInfo) {
  Key key;
  key.set(kbuf, klen);
  auto tid = rowex->lookup(key, *epocheInfo);
  if (tid == 0) {
    return false;
  }
  auto pair = reinterpret_cast<pair_string_t *>(tid);
  *value = pair->value;
  return true;
}
