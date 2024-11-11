using namespace geode::prelude;

extern "C" {
    #include "ffi.h"

    void log_debug(const char *str) { log::debug("{}", str); }
    void log_info(const char *str) { log::info("{}", str); }
    void log_warn(const char *str) { log::warn("{}", str); }
    void log_error(const char *str) { log::error("{}", str); }
}