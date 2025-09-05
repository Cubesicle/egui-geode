#pragma once

namespace egui_geode {
    void context(void (*reader)(const void *ctx));
    void add_run_fn(void (*run_fn)(const void *ctx));
}
