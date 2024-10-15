#pragma once

#ifdef GEODE_IS_WINDOWS
    #ifdef CUBESICLE_EGUI_API_EXPORTING
        #define EGUI_API_DLL __declspec(dllexport)
    #else
        #define EGUI_API_DLL __declspec(dllimport)
    #endif
#else
    #define EGUI_API_DLL __attribute__((visibility("default")))
#endif

namespace egui_api {
    EGUI_API_DLL void add_run_fn(void (*run_fn)(const void *ctx));
}