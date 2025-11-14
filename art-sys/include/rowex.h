#include <memory>
#include "tbb/tbb.h"
#include "ROWEX/Tree.h"
#include "Epoche.h"

using Rowex = ART_ROWEX::Tree;
using EpochInfo = ART::ThreadInfo;

std::unique_ptr<Rowex> rowex_new();

std::unique_ptr<EpochInfo> rowex_info(Rowex* rowex);

void rowex_insert(Rowex* rowex, uint64_t key, EpochInfo* epocheInfo);
