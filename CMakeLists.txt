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
    GIT_REPOSITORY https://github.com/jschwe/corrosion.git
    GIT_TAG 0a2758a9324ce2d41f7ec45cc4b48b4bbe8fc376
)
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(
    MANIFEST_PATH Cargo.toml
    CRATES ffi
)

corrosion_add_target_rustflags(ffi "$ENV{RUSTFLAGS} -Zstaticlib-allow-rdylib-deps=y -Zstaticlib-prefer-dynamic=y")

corrosion_experimental_cbindgen(
    TARGET ffi
    HEADER_NAME egui-geode.h
)

add_library(${PROJECT_NAME} SHARED
    src/main.cpp
    src/api.cpp
    src/gl-util.cpp
    src/ffi.cpp
)

target_include_directories(${PROJECT_NAME} PRIVATE include)

target_link_libraries(${PROJECT_NAME} ffi)
if (ANDROID)
    target_link_libraries(${PROJECT_NAME} EGL)
endif()

if (PROJECT_IS_TOP_LEVEL)
    target_compile_definitions(${PROJECT_NAME} PRIVATE CUBESICLE_EGUI_API_EXPORTING)
endif()

if (NOT DEFINED ENV{GEODE_SDK})
    message(FATAL_ERROR "Unable to find Geode SDK! Please define GEODE_SDK environment variable to point to Geode")
else()
    message(STATUS "Found Geode: $ENV{GEODE_SDK}")
endif()

add_subdirectory($ENV{GEODE_SDK} ${CMAKE_CURRENT_BINARY_DIR}/geode)

setup_geode_mod(${PROJECT_NAME})