#include "api.hpp"
#include "egui_geode_rust.h"

void egui_api::add_run_fn(void (*run_fn)(const void *ctx)) {
    gui_add_run_fn(run_fn);
}