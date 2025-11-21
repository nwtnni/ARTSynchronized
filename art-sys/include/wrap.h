#include <memory>

#include "Epoche.h"
#include "ROWEX/Tree.h"
#include "tbb/tbb.h"

using Rowex = ART_ROWEX::Tree;
using EpochInfo = ART::ThreadInfo;

std::unique_ptr<EpochInfo> rowex_info(Rowex *rowex);

std::unique_ptr<Rowex> rowex_u64_new();
bool rowex_u64_insert(Rowex *rowex, uint64_t key, uint64_t value,
                      EpochInfo *epocheInfo);
bool rowex_u64_lookup(Rowex *rowex, uint64_t key, uint64_t *value,
                      EpochInfo *epocheInfo);
bool rowex_u64_lookup_range(Rowex *rowex, uint64_t start, uint64_t end,
                            uint64_t result[], size_t resultSize,
                            size_t *resultsFound, EpochInfo *epocheInfo);

std::unique_ptr<Rowex> rowex_string_new();
bool rowex_string_insert(Rowex *rowex, const char *kbuf, size_t klen,
                         uint64_t value, EpochInfo *epocheInfo);
bool rowex_string_lookup(Rowex *rowex, const char *kbuf, size_t klen,
                         uint64_t *value, EpochInfo *epocheInfo);
