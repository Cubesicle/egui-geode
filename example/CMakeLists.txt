cmake_minimum_required(VERSION 3.21)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_OSX_ARCHITECTURES "arm64;x86_64")
set(CMAKE_CXX_VISIBILITY_PRESET hidden)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

project(EguiGeodeExample VERSION 1.0.0)

include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.5 # Optionally specify a commit hash, version tag or branch here
)
FetchContent_MakeAvailable(Corrosion)

# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(MANIFEST_PATH ../Cargo.toml CRATES exampleffi)

corrosion_add_target_rustflags(exampleffi "$ENV{RUSTFLAGS} -Zstaticlib-allow-rdylib-deps=y -Zstaticlib-prefer-dynamic=y")

corrosion_experimental_cbindgen(
    TARGET exampleffi
    HEADER_NAME rust-ffi.h
)

add_library(${PROJECT_NAME} SHARED
    src/main.cpp
    # Add any extra C++ source files here
)

target_link_libraries(${PROJECT_NAME} exampleffi)

if (NOT DEFINED ENV{GEODE_SDK})
    message(FATAL_ERROR "Unable to find Geode SDK! Please define GEODE_SDK environment variable to point to Geode")
else()
    message(STATUS "Found Geode: $ENV{GEODE_SDK}")
endif()

add_subdirectory($ENV{GEODE_SDK} ${CMAKE_CURRENT_BINARY_DIR}/geode)

setup_geode_mod(${PROJECT_NAME})