#include <memory>

#include "Epoche.h"
#include "ROWEX/Tree.h"
#include "tbb/tbb.h"

using Rowex = ART_ROWEX::Tree;
using EpochInfo = ART::ThreadInfo;

std::unique_ptr<Rowex> rowex_new();

std::unique_ptr<EpochInfo> rowex_info(Rowex *rowex);

void rowex_insert(Rowex *rowex, uint64_t key, EpochInfo *epocheInfo);

uint64_t rowex_lookup(Rowex *rowex, uint64_t key, EpochInfo *epocheInfo);

void rowex_remove(Rowex *rowex, uint64_t key, EpochInfo *epocheInfo);
