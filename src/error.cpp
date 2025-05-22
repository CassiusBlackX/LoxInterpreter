#include "error.h"
#include <cstddef>
#include <cstdlib>
#include <iostream>

void error(size_t line, const std::string& message) {
  std::cerr << "[Line: " << line << "] Error: " << message << std::endl;
  exit(1);
}
