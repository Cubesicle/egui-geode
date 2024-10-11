#include "rusty.h"
#include "gl_util.hpp"
using namespace geode::prelude;

$on_mod(Loaded) {
    init_context();
    run_in_context([]() {
        init_gui();
    });
}

#include <Geode/modify/PlayLayer.hpp>
class $modify(PlayLayer) {
    void startGame() {
        bingus(getNonVirtual(this), getNonVirtual(&PlayLayer::startGame));
    }
};