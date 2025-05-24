#include "error.h"
#include <cstddef>
#include <cstdlib>
#include <iostream>

bool had_error;
void error(size_t line, const std::string& message) {
  std::cerr << "[Line: " << line << "] Error: " << message << std::endl;
  had_error = true;
  exit(1);
}
