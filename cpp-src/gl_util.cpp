#include "gl_util.hpp"

#ifdef GEODE_IS_WINDOWS
    const uintptr_t gl_get_proc_address(const char *str) {
        auto fn = wglGetProcAddress(str);
        if (fn == NULL) fn = GetProcAddress(GetModuleHandleA("opengl32.dll"), str);
        return reinterpret_cast<uintptr_t>(fn);
    }

    void init_context() {
        new_context = wglCreateContext(wglGetCurrentDC());
    }

    void run_in_context(std::function<void()> const &fn) {
        const auto old_ctx = wglGetCurrentContext();
        wglMakeCurrent(wglGetCurrentDC(), new_context);
        fn();
        wglMakeCurrent(wglGetCurrentDC(), old_ctx);
    }
#else
    const uintptr_t gl_get_proc_address(const char *str) {
        return reinterpret_cast<const void *>(eglGetProcAddress(str));
    }

    void init_context() {
        const auto display = eglGetCurrentDisplay();
        const EGLint attrib_list[] = {EGL_CONTEXT_CLIENT_VERSION, 2, EGL_NONE};

        new_context = eglCreateContext(display, NULL, EGL_NO_CONTEXT, attrib_list);
    }

    void run_in_context(std::function<void()> const &fn) {
        const auto display = eglGetCurrentDisplay();
        const auto draw = eglGetCurrentSurface(EGL_DRAW);
        const auto read = eglGetCurrentSurface(EGL_READ);
        const auto old_ctx = eglGetCurrentContext();

        eglMakeCurrent(display, draw, read, new_context);
        fn();
        eglMakeCurrent(display, draw, read, old_ctx);
    }
#endif