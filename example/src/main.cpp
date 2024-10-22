#include "cubesicle.egui-api/include/api.hpp"
#include "rust-ffi.h"
using namespace geode::prelude;

$on_mod(Loaded) {
    egui_api::add_run_fn(run_fn);
}

#include <Geode/modify/PlayLayer.hpp>
class $modify(PlayLayer) {
    void startGame() {
        bingus(getNonVirtual(this), getNonVirtual(&PlayLayer::startGame));
    }
};