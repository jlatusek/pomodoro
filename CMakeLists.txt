cmake_minimum_required(VERSION 3.31)
project(bet_game)

enable_testing()
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(CMAKE_CXX_STANDARD 26)

add_library(pomodoro_lib
        ${CMAKE_CURRENT_SOURCE_DIR}/src/pomodoro.cpp
)
target_include_directories(pomodoro_lib PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)

add_executable(pomodoro ${CMAKE_CURRENT_SOURCE_DIR}/src/main.cpp)
target_link_libraries(pomodoro pomodoro_lib)

include(GoogleTest)
find_package(GTest REQUIRED)

add_executable(pomodoro-tests
        ${CMAKE_CURRENT_SOURCE_DIR}/tests/tests.cpp
)
target_link_libraries(pomodoro-tests GTest::gtest GTest::gtest_main pomodoro_lib)

gtest_discover_tests(pomodoro-tests)
