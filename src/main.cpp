#include "Rusty.h"

using namespace geode::prelude;

HGLRC new_ctx;

void run_in_ctx(HGLRC ctx, std::function<void()> const& fn) {
    const auto old_ctx = wglGetCurrentContext();
    wglMakeCurrent(wglGetCurrentDC(), ctx);
    fn();
    wglMakeCurrent(wglGetCurrentDC(), old_ctx);
}

// Code taken from https://github.com/matcool/gd-imgui-cocos
std::tuple<float, float> convert_cocos_point(CCPoint point) {
    const auto director = CCDirector::sharedDirector();
    const auto window_size = director->getWinSize();
    const auto frame_size = director->getOpenGLView()->getFrameSize() * utils::getDisplayFactor();
    return {
        point.x / window_size.width * frame_size.width,
        (1.f - point.y / window_size.height) * frame_size.height
    };
}

$on_mod(Loaded) {
    new_ctx = wglCreateContext(wglGetCurrentDC());
    run_in_ctx(new_ctx, []() {
        init_gui();
    });
}

#include <Geode/modify/CCTouchDispatcher.hpp>
class $modify(CCTouchDispatcher) {
	void touches(CCSet *touches, CCEvent *event, unsigned int type) {
        if (!gui_wants_pointer_input()) {
            CCTouchDispatcher::touches(touches, event, type);
            return;
        }

        auto *touch = static_cast<CCTouch*>(touches->anyObject());
        const auto touch_pos = convert_cocos_point(touch->getLocation());
        if (type == CCTOUCHBEGAN) {
            gui_send_mouse_btn(
                std::get<0>(touch_pos),
                std::get<1>(touch_pos),
                false,
                true
            );
            return;
        } else if (type == CCTOUCHENDED || type == CCTOUCHCANCELLED) {
            gui_send_mouse_btn(
                std::get<0>(touch_pos),
                std::get<1>(touch_pos),
                false,
                false
            );
        }

        CCTouchDispatcher::touches(touches, event, type);
    }
};

#include <Geode/modify/CCEGLView.hpp>
class $modify(CCEGLView) {
    void swapBuffers() {
        const auto mouse_pos = convert_cocos_point(cocos::getMousePos());
        gui_send_mouse_pos(std::get<0>(mouse_pos), std::get<1>(mouse_pos));

        const auto frame_size = getFrameSize();
        run_in_ctx(new_ctx, [frame_size]() {
            swap_buffers_detour(frame_size.width, frame_size.height);
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