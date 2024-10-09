#pragma once

#ifdef GEODE_IS_WINDOWS
    typedef HGLRC GLContext;
#else
    #include <EGL/egl.h>

    typedef EGLContext GLContext;
#endif

extern "C" const void *gl_get_proc_address(const char *str);
GLContext gl_create_context();
void run_in_context(GLContext ctx, std::function<void()> const &fn);