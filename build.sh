#!/usr/bin/env bash
case $1 in
    "" | "windows")
        splat_dir=~/.local/share/Geode/cross-tools/splat
        toolchain=~/.local/share/Geode/cross-tools/clang-msvc-sdk/clang-cl-msvc.cmake
        geode build -p windows -- -DCMAKE_TOOLCHAIN_FILE=$toolchain -DSPLAT_DIR=$splat_dir -DRust_CARGO_TARGET=x86_64-pc-windows-msvc ;;
    "android" | "android32" | "android64")
        if [[ -z $ANDROID_NDK_ROOT ]]; then
            export ANDROID_NDK_ROOT=~/.local/share/Geode/cross-tools/android-ndk
        fi;&
    *)
        geode build -p $1 --config RelWithDebInfo ;;
esac