#include <memory>

#include "Epoche.h"
#include "ROWEX/Tree.h"
#include "tbb/tbb.h"

using Rowex = ART_ROWEX::Tree;
using EpochInfo = ART::ThreadInfo;

std::unique_ptr<EpochInfo> rowex_info(Rowex *rowex);

std::unique_ptr<Rowex> rowex_u64_new();
void rowex_u64_insert(Rowex *rowex, TID tid, EpochInfo *epocheInfo);
TID rowex_u64_lookup(Rowex *rowex, TID tid, EpochInfo *epocheInfo);
void rowex_u64_remove(Rowex *rowex, TID tid, EpochInfo *epocheInfo);

std::unique_ptr<Rowex> rowex_string_new();
void rowex_string_insert(Rowex *rowex, TID tid, EpochInfo *epocheInfo);
TID rowex_string_lookup(Rowex *rowex, TID tid, EpochInfo *epocheInfo);
void rowex_string_remove(Rowex *rowex, TID tid, EpochInfo *epocheInfo);
