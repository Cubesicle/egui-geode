#include "egui_geode_rust.h"
#include "gl_util.hpp"
using namespace geode::prelude;

struct EguiPos2 {
    float x;
    float y;
};

bool key_is_modifier(enumKeyCodes key);
bool key_is_controller(enumKeyCodes key);
EguiPos2 convert_cocos_point(const CCPoint &point);

$on_mod(Loaded) {
    init_context();
    run_in_context([]() {
        init_gui();
    });
}

#include <Geode/modify/CCKeyboardDispatcher.hpp>
class $modify(CCKeyboardDispatcher) {
    void updateModifierKeys(bool shift, bool ctrl, bool alt, bool cmd) {
        #ifdef GEODE_IS_MACOS
            gui_send_modifiers(shift, ctrl, alt, cmd, cmd);
        #else
            gui_send_modifiers(shift, ctrl, alt, cmd, ctrl);
        #endif
        
        CCKeyboardDispatcher::updateModifierKeys(shift, ctrl, alt, cmd);
    }
    
    bool dispatchKeyboardMSG(enumKeyCodes key, bool down, bool repeat) {
        if (key != KEY_Unknown && key != KEY_None && !key_is_modifier(key) && !key_is_controller(key)) {
            gui_send_key_press(CCKeyboardDispatcher::keyToString(key), down, repeat);
        }
        if (gui_wants_keyboard_input()) return false;

        return CCKeyboardDispatcher::dispatchKeyboardMSG(key, down, repeat);
    }
};

#include <Geode/modify/CCIMEDispatcher.hpp>
class $modify(CCIMEDispatcher) {
    void dispatchInsertText(const char *text, int len, enumKeyCodes keys) {
        if (gui_wants_keyboard_input()) {
            gui_send_text_input(text);
        } else {
            CCIMEDispatcher::dispatchInsertText(text, len, keys);
        }
    }

    void dispatchDeleteBackward() {
        gui_send_key_press("Backspace", true, false);
        gui_send_key_press("Backspace", false, false);

        CCIMEDispatcher::dispatchDeleteBackward();
    }

    void dispatchDeleteForward() {
        gui_send_key_press("Delete", true, false);
        gui_send_key_press("Delete", false, false);

        CCIMEDispatcher::dispatchDeleteForward();
    }
};

#include <Geode/modify/CCTouchDispatcher.hpp>
class $modify(CCTouchDispatcher) {
    void touches(CCSet *touches, CCEvent *event, unsigned int type) {
        const auto touch = static_cast<CCTouch *>(touches->anyObject());
        const auto touch_pos = convert_cocos_point(touch->getLocation());
        gui_send_touch(touch->getID(), type, touch_pos.x, touch_pos.y);
        if (type == CCTOUCHBEGAN && is_pos_over_gui_area(touch_pos.x, touch_pos.y)) return;

        CCTouchDispatcher::touches(touches, event, type);
    }
};

#include <Geode/modify/CCMouseDispatcher.hpp>
class $modify(CCMouseDispatcher) {
    bool dispatchScrollMSG(float x, float y) {
        gui_send_scroll_event(y * 5, -x * 5);
        if (gui_wants_pointer_input()) return true;

        return CCMouseDispatcher::dispatchScrollMSG(x, y);
    }
};

#include <Geode/modify/CCEGLView.hpp>
class $modify(CCEGLView) {
    void swapBuffers() {
        #ifdef GEODE_IS_DESKTOP
            const auto mouse_pos = convert_cocos_point(cocos::getMousePos());
            gui_send_mouse_pos(mouse_pos.x, mouse_pos.y);
        #endif

        const auto frame_size = getFrameSize();
        run_in_context([frame_size]() {
            swap_buffers_detour(frame_size.width, frame_size.height);
        });

        CCEGLView::swapBuffers();
    }
};

bool key_is_modifier(enumKeyCodes key) {
    return
        key == KEY_Control ||
        key == KEY_LeftControl ||
        key == KEY_RightContol ||
        key == KEY_Shift ||
        key == KEY_LeftShift ||
        key == KEY_RightShift ||
        key == KEY_Alt ||
        key == KEY_LeftWindowsKey ||
        key == KEY_RightWindowsKey;
}

bool key_is_controller(enumKeyCodes key) {
    return key >= CONTROLLER_A && key <= CONTROLLER_RTHUMBSTICK_RIGHT;
}

// Code taken from https://github.com/matcool/gd-imgui-cocos
EguiPos2 convert_cocos_point(const CCPoint &point) {
    const auto director = CCDirector::sharedDirector();
    const auto window_size = director->getWinSize();
    const auto frame_size = director->getOpenGLView()->getFrameSize() * utils::getDisplayFactor();
    return {
        point.x / window_size.width * frame_size.width,
        (1.f - point.y / window_size.height) * frame_size.height
    };
}