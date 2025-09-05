cmake_minimum_required(VERSION 3.21)

project(EguiGeode VERSION 1.0.0)

include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.5
)
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(
    MANIFEST_PATH Cargo.toml
    CRATES egui-geode-rust
)

corrosion_experimental_cbindgen(
    TARGET egui_geode_rust
    HEADER_NAME egui-geode-rust.h
)

add_library(${PROJECT_NAME} INTERFACE)

set_target_properties(${PROJECT_NAME} PROPERTIES CXX_VISIBILITY_PRESET hidden)

file(GLOB SOURCE_FILES "cpp-src/*")
target_sources(${PROJECT_NAME} INTERFACE ${SOURCE_FILES})

target_include_directories(${PROJECT_NAME} INTERFACE include)

target_link_libraries(${PROJECT_NAME} INTERFACE egui_geode_rust)
if (ANDROID)
    target_link_libraries(${PROJECT_NAME} INTERFACE EGL)
endif()
