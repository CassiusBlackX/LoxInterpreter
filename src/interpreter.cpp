#include "interpreter.h"
#include "error.h"

void Interpreter::interpret(const std::vector<Stmt *> statements) {
  try {
    for (Stmt *statement : statements) {
      statement->execute(&environment);
    }
  } catch (const RuntimeError &e) {
    handle_runtime_error(e);
  }
}
