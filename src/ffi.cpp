using namespace geode::prelude;

extern "C" {
    #include "ffi.h"

    void _log_debug(char *str) { log::debug("{}", str); }
    void _log_info(char *str) { log::info("{}", str); }
    void _log_warn(char *str) { log::warn("{}", str); }
    void _log_error(char *str) { log::error("{}", str); }
}