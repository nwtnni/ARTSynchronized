#include <memory>

#include "Epoche.h"
#include "ROWEX/Tree.h"
#include "tbb/tbb.h"

using Rowex = ART_ROWEX::Tree;
using EpochInfo = ART::ThreadInfo;

std::unique_ptr<EpochInfo> rowex_info(Rowex *rowex);
std::unique_ptr<Rowex> rowex_u64_new();
void rowex_u64_insert(Rowex *rowex, uint64_t key, EpochInfo *epocheInfo);
TID rowex_u64_lookup(Rowex *rowex, uint64_t key, EpochInfo *epocheInfo);
void rowex_u64_remove(Rowex *rowex, uint64_t key, EpochInfo *epocheInfo);
