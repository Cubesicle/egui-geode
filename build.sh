#!/usr/bin/env bash
case $1 in
    "" | "windows")
        splat_dir=~/.local/share/Geode/cross-tools/splat
        toolchain=~/.local/share/Geode/cross-tools/clang-msvc-sdk/clang-cl-msvc.cmake

        export CC_x86_64_pc_windows_msvc="clang-cl"
        export CXX_x86_64_pc_windows_msvc="clang-cl"
        export AR_x86_64_pc_windows_msvc="llvm-lib"
        export CL_FLAGS="-Wno-unused-command-line-argument -fuse-ld=lld-link $splat_dir/crt/include $splat_dir/sdk/include/ucrt $splat_dir/sdk/include/um $splat_dir/sdk/include/shared"
        export CFLAGS_x86_64_pc_windows_msvc="$CL_FLAGS"
        export CXXFLAGS_x86_64_pc_windows_msvc="$CL_FLAGS"
        export RUSTFLAGS="-Lnative=$splat_dir/crt/lib/x86_64 -Lnative=$splat_dir/sdk/lib/um/x86_64 -Lnative=$splat_dir/sdk/lib/ucrt/x86_64"
        export CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER="lld-link"

        geode build -p windows -- -DCMAKE_TOOLCHAIN_FILE=$toolchain -DSPLAT_DIR=$splat_dir ;;
    "android" | "android32" | "android64")
        if [[ -z $ANDROID_NDK_ROOT ]]; then
            export ANDROID_NDK_ROOT=~/.local/share/Geode/cross-tools/android-ndk
        fi ;&
    *)
        geode build -p $1 --config RelWithDebInfo ;;
esac
