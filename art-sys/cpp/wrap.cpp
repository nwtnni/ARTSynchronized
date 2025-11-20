#include "wrap.h"
#include <iostream>

static void load_u64(TID tid, Key &key) {
  // Store the key of the tuple into the key vector
  // Implementation is database specific
  key.setKeyLen(sizeof(tid));
  reinterpret_cast<uint64_t *>(&key[0])[0] = __builtin_bswap64(tid);
}

std::unique_ptr<EpochInfo> rowex_info(Rowex *rowex) {
  return std::make_unique<EpochInfo>(rowex->getThreadInfo());
}

std::unique_ptr<Rowex> rowex_u64_new() {
  return std::make_unique<Rowex>(load_u64);
}

void rowex_u64_insert(Rowex *rowex, uint64_t tid, EpochInfo *epocheInfo) {
  Key key;
  load_u64(tid, key);
  return rowex->insert(key, tid, *epocheInfo);
}

TID rowex_u64_lookup(Rowex *rowex, uint64_t tid, EpochInfo *epocheInfo) {
  Key key;
  load_u64(tid, key);
  return rowex->lookup(key, *epocheInfo);
}

void rowex_u64_remove(Rowex *rowex, uint64_t tid, EpochInfo *epocheInfo) {
  Key key;
  load_u64(tid, key);
  return rowex->remove(key, tid, *epocheInfo);
}

static void load_string(TID tid, Key &key) {
  uint64_t len = tid >> 48;
  const char *ptr = reinterpret_cast<char *>(tid & ((UINT64_C(1) << 48) - 1));
  key.set(ptr, len);
}

std::unique_ptr<Rowex> rowex_string_new() {
  return std::make_unique<Rowex>(load_string);
}

void rowex_string_insert(Rowex *rowex, TID tid, EpochInfo *epocheInfo) {
  Key key;
  load_string(tid, key);
  return rowex->insert(key, tid, *epocheInfo);
}

TID rowex_string_lookup(Rowex *rowex, TID tid, EpochInfo *epocheInfo) {
  Key key;
  load_string(tid, key);
  return rowex->lookup(key, *epocheInfo);
}

void rowex_string_remove(Rowex *rowex, TID tid, EpochInfo *epocheInfo) {
  Key key;
  load_string(tid, key);
  return rowex->remove(key, tid, *epocheInfo);
}
