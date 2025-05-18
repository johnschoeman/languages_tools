cmake_minimum_required(VERSION 3.10)

project(MyHttp VERSION 1.0)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)

set(SOURCES src/server.cpp src/http_tcpServer.cpp src/http_tcpServer.h)
add_executable(MyHttp src/server.cpp src/http_tcpServer.cpp)

target_include_directories(MyHttp PUBLIC
                           "${PROJECT_BINARY_DIR}"
                           )

