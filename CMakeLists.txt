cmake_minimum_required(VERSION 3.20)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# set c++ compiler
if(NOT DEFINED CMAKE_CXX_COMPILER)
  find_program(CLANG_COMPILER clang++)
  if(CLANG_COMPILER)
    set(CMAKE_CXX_COMPILER clang++)
    set(CMAKE_C_COMPILER clang)
    message(STATUS "Using clang compiler: ${CMAKE_CXX_COMPILER}")
  else()
    find_program(GCC_COMPILER g++)
    if(GCC_COMPILER)
      set(CMAKE_CXX_COMPILER g++)
      set(CMAKE_C_COMPILER gcc)
      message(STATUS "Using GCC compiler: ${CMAKE_CXX_COMPILER}")
    elseif(MSVC)
      message(STATUS "Using MSVC compiler (default)")
    else()
      message(ERROR "No supported compiler found (Clang/GCC/MSVC)")
    endif()
  endif()
endif()

project(cpp_lox)

option(BUILD_TESTS "Build tests" OFF)

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/src/)

set(SOURCE_FILES
  src/error.cpp
  src/token.cpp
  src/object.cpp
  src/scanner.cpp
  src/expr.cpp
  src/parser.cpp
  src/interpreter.cpp
  src/stmt.cpp
  src/environment.cpp
  src/function.cpp
)

add_definitions(
  -DFLOAT_PRECISION=3
  -DCMAKELISTS_PATH="${CMAKE_CURRENT_SOURCE_DIR}"
)

add_executable(${PROJECT_NAME} src/main.cpp ${SOURCE_FILES})

if(BUILD_TESTS)
  enable_testing()
  find_package(GTest REQUIRED)
  include_directories(${GTest_INCLUDE_DIRS})
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
else()
  message(STATUS "Skipping tests (use -BUILD_TESTS=ON to enable)")
endif()
