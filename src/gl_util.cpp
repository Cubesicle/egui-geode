#include "gl_util.hpp"

#ifdef GEODE_IS_WINDOWS
    extern "C" const void *gl_get_proc_address(const char *str) {
        auto fn = wglGetProcAddress(str);
        if (fn == NULL) fn = GetProcAddress(GetModuleHandleA("opengl32.dll"), str);
        return reinterpret_cast<const void *>(fn);
    }

    GLContext gl_create_context() {
        return wglCreateContext(wglGetCurrentDC());
    }

    void run_in_context(GLContext ctx, std::function<void()> const &fn) {
        const auto old_ctx = wglGetCurrentContext();
        wglMakeCurrent(wglGetCurrentDC(), ctx);
        fn();
        wglMakeCurrent(wglGetCurrentDC(), old_ctx);
    }
#else
    extern "C" const void *gl_get_proc_address(const char *str) {
        return reinterpret_cast<const void *>(eglGetProcAddress(str));
    }

    GLContext gl_create_context() {
        return eglGetCurrentContext();
    }

    void run_in_context(GLContext ctx, std::function<void()> const &fn) {
        fn();
    }
#endif