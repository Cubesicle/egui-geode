#include "Rusty.h"

using namespace geode::prelude;

HGLRC new_ctx;

void run_in_ctx(HGLRC ctx, std::function<void()> const& fn) {
    auto old_ctx = wglGetCurrentContext();
    wglMakeCurrent(wglGetCurrentDC(), ctx);
    fn();
    wglMakeCurrent(wglGetCurrentDC(), old_ctx);
}

$on_mod(Loaded) {
    new_ctx = wglCreateContext(wglGetCurrentDC());
    run_in_ctx(new_ctx, []() {
        init_gui();
    });
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
        auto frame_size = this->getFrameSize();
        run_in_ctx(new_ctx, [frame_size]() {
            swap_buffers_detour(frame_size.width, frame_size.height);
        });

        CCEGLView::swapBuffers();
    }
};