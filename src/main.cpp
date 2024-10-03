#include "Rusty.h"

using namespace geode::prelude;

HGLRC new_ctx;

void run_in_ctx(HGLRC ctx, std::function<void()> const& fn) {
    const auto old_ctx = wglGetCurrentContext();
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

#include <Geode/modify/CCEGLView.hpp>
class $modify(CCEGLView) {
    void swapBuffers() {
        const auto window_size = CCDirector::sharedDirector()->getWinSize();
        const auto frame_size = this->getFrameSize() * utils::getDisplayFactor();
        const auto mouse_pos = cocos::getMousePos();
        run_in_ctx(new_ctx, [window_size, frame_size, mouse_pos]() {
            swap_buffers_detour(
                frame_size.width,
                frame_size.height,
                mouse_pos.x / window_size.width * frame_size.width,
                (1.f - mouse_pos.y / window_size.height) * frame_size.height
            );
        });

        CCEGLView::swapBuffers();
    }
};

#include <Geode/modify/PlayLayer.hpp>
class $modify(PlayLayer) {
    void startGame() {
        bingus(getNonVirtual(this), getNonVirtual(&PlayLayer::startGame));
    }
};