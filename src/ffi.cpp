using namespace geode::prelude;

extern "C" {
    #include "ffi.h"

    void log_debug(char *str) { log::debug("{}", str); }
    void log_info(char *str) { log::info("{}", str); }
    void log_warn(char *str) { log::warn("{}", str); }
    void log_error(char *str) { log::error("{}", str); }
    const void *gl_get_proc_address(const char *str) {
        auto fn = wglGetProcAddress(str);
        if (fn == NULL) fn = GetProcAddress(GetModuleHandleA("opengl32.dll"), str);
        return reinterpret_cast<const void *>(fn);
    }
}