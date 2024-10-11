cmake_minimum_required(VERSION 3.21)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_OSX_ARCHITECTURES "arm64;x86_64")
set(CMAKE_CXX_VISIBILITY_PRESET hidden)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

project(EguiGeode VERSION 1.0.0)

include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.5 # Optionally specify a commit hash, version tag or branch here
)
FetchContent_MakeAvailable(Corrosion)

# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(MANIFEST_PATH Cargo.toml)

corrosion_experimental_cbindgen(
    TARGET egui_geode_rust
    HEADER_NAME egui_geode_rust.h
)

# Set up the mod binary
add_library(${PROJECT_NAME} SHARED src/main.cpp src/gl_util.cpp src/ffi.cpp)

if (ANDROID)
    target_link_libraries(${PROJECT_NAME} egui_geode_rust EGL)
else()
    target_link_libraries(${PROJECT_NAME} egui_geode_rust)
endif()