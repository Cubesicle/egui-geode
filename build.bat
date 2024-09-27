@echo off
cmake -DCMAKE_BUILD_TYPE=Release -B build
cmake --build build --config Release