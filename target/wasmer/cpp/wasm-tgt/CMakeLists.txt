cmake_minimum_required(VERSION 3.15)
set (PROJECT_NAME wasm-tgt)

project(${PROJECT_NAME})

set(CMAKE_CXX_STANDARD 14)

add_executable(${PROJECT_NAME} run-tgt.cpp)

target_include_directories(${PROJECT_NAME} PUBLIC $ENV{WASMER_DIR}/include)
target_link_libraries(${PROJECT_NAME} $ENV{WASMER_DIR}/lib/libwasmer.a)

find_package(Threads REQUIRED)
target_link_libraries(${PROJECT_NAME} Threads::Threads)

find_library(dl NAMES dl PATH /usr/lib/x86_64-linux-gnu/libdl.so)
target_link_libraries(${PROJECT_NAME} ${dl})
