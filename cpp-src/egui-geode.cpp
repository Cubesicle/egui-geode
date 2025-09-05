#include "egui-geode.hpp"
#include "egui-geode-rust.h"

namespace egui_geode {
    void context(void (*reader)(const void *ctx)) {
        gui_context(reader);
    }

    void add_run_fn(void (*run_fn)(const void *ctx)) {
        gui_add_run_fn(run_fn);
    }
}
