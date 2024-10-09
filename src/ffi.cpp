using namespace geode::prelude;

extern "C" {
    #include "ffi.h"

    void log_debug(char *str) { log::debug("{}", str); }
    void log_info(char *str) { log::info("{}", str); }
    void log_warn(char *str) { log::warn("{}", str); }
    void log_error(char *str) { log::error("{}", str); }
}