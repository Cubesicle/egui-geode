#include "ffi.h"

using namespace geode::prelude;

void _log_info(char *str) {
    log::info("{}", str);
}