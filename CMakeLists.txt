cmake_minimum_required(VERSION 3.20)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_COMPILER clang++)
set(CMAKE_C_COMPILER clang)
set(CMAKE_BUILD_TYPE Debug)

project(cpp_lox)
enable_testing()

find_package(GTest REQUIRED)
include_directories(${GTest_INCLUDE_DIRS})

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/src/)

set(SOURCE_FILES
  src/error.cpp
  src/token.cpp
  src/scanner.cpp
  src/expr.cpp
  src/parser.cpp
  src/interpreter.cpp
  src/stmt.cpp
  src/environment.cpp
)

add_definitions(
  -DFLOAT_PRECISION=3
  -DCMAKELISTS_PATH="${CMAKE_CURRENT_SOURCE_DIR}"
)

add_executable(${PROJECT_NAME} src/main.cpp ${SOURCE_FILES})

# tests
# token test
add_executable(token_test tests/token_test.cpp ${SOURCE_FILES})
target_link_libraries(token_test PRIVATE GTest::GTest GTest::Main)
gtest_discover_tests(token_test)
# scanner test
add_executable(scanner_test tests/scanner_test.cpp ${SOURCE_FILES})
target_link_libraries(scanner_test PRIVATE GTest::GTest GTest::Main)
gtest_discover_tests(scanner_test)
# expr test
add_executable(expr_test tests/expr_test.cpp ${SOURCE_FILES})
target_link_libraries(expr_test PRIVATE GTest::GTest GTest::Main)
gtest_discover_tests(expr_test)
