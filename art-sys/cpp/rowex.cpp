#include <memory>
#include <iostream>

#include "rowex.h"

void loadKey(TID tid, Key &key) {
    // Store the key of the tuple into the key vector
    // Implementation is database specific
    key.setKeyLen(sizeof(tid));
    reinterpret_cast<uint64_t *>(&key[0])[0] = __builtin_bswap64(tid);
}

std::unique_ptr<ART_ROWEX::Tree> rowex_new() {
    return std::unique_ptr<ART_ROWEX::Tree>(new ART_ROWEX::Tree(loadKey));
}

std::unique_ptr<EpochInfo> rowex_info(Rowex* rowex) {
    return std::make_unique<EpochInfo>(std::move(rowex->getThreadInfo()));
}

void rowex_insert(Rowex* rowex, uint64_t key, EpochInfo* epocheInfo) {
    std::cerr << "insert " << key << std::endl;
}
