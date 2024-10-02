#include "Geode/c++stl/string.hpp"
#include "Geode/cocos/platform/win32/CCEGLView.h"
#include "Rusty.h"
#include <WinDef.h>
#include <minwindef.h>
#include <wingdi.h>

using namespace geode::prelude;

PROC lol;

#include <Geode/modify/PlayLayer.hpp>
class $modify(PlayLayer) {
    void startGame() {
        bingus(getNonVirtual(this), getNonVirtual(&PlayLayer::startGame));
    }
};

#include <Geode/modify/CCDirector.hpp>
class $modify(CCDirector) {
    void drawScene() {
        CCDirector::drawScene();
        lol = wglGetProcAddress("swapBuffers");
    }
};

#include <Geode/modify/CCEGLView.hpp>
class $modify(CCEGLView) {
    void swapBuffers() {
        //swap_buffers_detour();
        std::cout << lol << std::endl;
        CCEGLView::swapBuffers();
    }
};