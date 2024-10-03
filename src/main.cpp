#include "Rusty.h"

using namespace geode::prelude;

$on_mod(Loaded) {
    init_gui();
}

#include <Geode/modify/PlayLayer.hpp>
class $modify(PlayLayer) {
    void startGame() {
        bingus(getNonVirtual(this), getNonVirtual(&PlayLayer::startGame));
    }
};

#include <Geode/modify/CCEGLView.hpp>
class $modify(CCEGLView) {
    void swapBuffers() {
        swap_buffers_detour();
        CCEGLView::swapBuffers();
    }
};