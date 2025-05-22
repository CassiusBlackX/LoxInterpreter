cmake_minimum_required(VERSION 3.20)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_COMPILER clang++)
set(CMAKE_C_COMPILER clang)
set(CMAKE_BUILD_TYPE Debug)

project(cpp_loxs)
enable_testing()

find_package(GTest REQUIRED)
include_directories(${GTest_INCLUDE_DIRS})

# tests
file(GLOB TEST_FILES "tests/*.cpp")
add_executable(${PROJECT_NAME}_tests ${TEST_FILES})
target_link_libraries(${PROJECT_NAME}_tests GTest::GTest GTest::Main)

add_executable(${PROJECT_NAME}
  src/main.cpp
  src/error.cpp
  src/token.cpp
  src/scanner.cpp
)
