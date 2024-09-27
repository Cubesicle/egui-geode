extern "C" {
    #include "ffi.h"
}

using namespace geode::prelude;

extern "C" {
    void _log_info(char *str) {
        log::info("{}", str);
    }
}