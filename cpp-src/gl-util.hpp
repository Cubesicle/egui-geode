#pragma once

#ifdef GEODE_IS_WINDOWS
    typedef HGLRC GLContext;
#else
    #include <EGL/egl.h>

    typedef EGLContext GLContext;
#endif
    
static GLContext new_context;
extern "C" const void *gl_get_proc_address(const char *str);
void init_context();
void run_in_context(std::function<void()> const &fn);