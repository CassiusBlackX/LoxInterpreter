#include "error.h"
#include <cstddef>
#include <cstdlib>
#include <iostream>

bool had_error = false;
void error(size_t line, const std::string& message) {
  std::cerr << "[Line: " << line << "] Error: " << message << std::endl;
  had_error = true;
}

bool had_runtime_error = false;
void handle_runtime_error(const RuntimeError& e) {
  std::cerr << e.what() << std::endl;
  had_runtime_error = false;
}
