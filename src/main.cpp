#include <cstdlib>
#include <filesystem>
#include <fstream>
#include <ios>
#include <iostream>
#include <ostream>
#include <stdexcept>
#include <vector>

#include "token.h"
#include "scanner.h"

bool had_error;

void run(std::string source) {
  Scanner scanner(source);
  std::vector<Token> tokens = scanner.scan_tokens();

  for (auto token : tokens) {
    std::cout << token << std::endl;
  }
}

void run_file(std::string path) {
  std::ifstream file(path, std::ios::binary);
  if (!file) {
    throw std::runtime_error("Failed to open file: " + path);
  }

  std::streamsize file_size = std::filesystem::file_size(path);

  std::vector<char> bytes(file_size);
  file.read(bytes.data(), file_size);
  file.close();

  std::string content(bytes.begin(), bytes.end());
  run(content);

  if (had_error)
    std::exit(65);
}

void run_prompt() {
  for (;;) {
    std::cout << "> " << std::flush;
    std::string line;
    if (!std::getline(std::cin, line))
      break;
    run(line);
    had_error = false;
  }
}

int main(int argc, char **argv) {
  if (argc > 1) {
    std::cerr << "Usage: cpp_lox [script]" << std::endl;
    std::exit(64);
  } else if (argc == 1) {
    run_file(argv[0]);
  } else {
    run_prompt();
  }
}
