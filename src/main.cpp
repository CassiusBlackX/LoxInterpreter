#include <cstdlib>
#include <filesystem>
#include <fstream>
#include <ios>
#include <iostream>
#include <ostream>
#include <stdexcept>
#include <vector>

#include "expr.h"
#include "interpreter.h"
#include "parser.h"
#include "scanner.h"
#include "stmt.h"
#include "token.h"

extern bool had_error;
extern bool had_runtime_error;

void run(const std::string &source) {
  Scanner scanner(source);
  std::vector<Token> tokens = scanner.scan_tokens();

  Parser parser(tokens);
  std::vector<Stmt *> statements = parser.parse();

  Interpreter interpreter;
  interpreter.interpret(statements);

  for (Stmt *statement : statements) {
    delete_stmt(statement);
  }

  if (had_error)
    return;
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
  if (had_runtime_error)
    std::exit(70);
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
  if (argc > 2) {
    std::exit(64);
  } else if (argc == 2) {
    run_file(argv[1]);
  } else {
    run_prompt();
  }
}
