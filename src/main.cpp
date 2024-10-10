#include "rusty.h"
#include "gl_util.hpp"
using namespace geode::prelude;

std::tuple<float, float> convert_cocos_point(CCPoint point);

$on_mod(Loaded) {
    init_context();
    run_in_context([]() {
        init_gui();
    });
}

#include <Geode/modify/CCTouchDispatcher.hpp>
class $modify(CCTouchDispatcher) {
	void touches(CCSet *touches, CCEvent *event, unsigned int type) {
        //for (auto it = touches->begin(); it != touches->end(); it++) {
        //    const auto touch = static_cast<CCTouch *>(*it);
        //    const auto touch_pos = convert_cocos_point(touch->getLocation());
        //    if (gui_send_touch(touch->getID(), type, std::get<0>(touch_pos), std::get<1>(touch_pos))) {
        //        touches->removeObject(touch);
        //    }
        //}
        const auto touch = static_cast<CCTouch *>(touches->anyObject());
        const auto touch_pos = convert_cocos_point(touch->getLocation());
        if (gui_send_touch(touch->getID(), type, std::get<0>(touch_pos), std::get<1>(touch_pos))) {
            return;
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
        run_in_context([frame_size]() {
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
