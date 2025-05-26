#include <vector>

#include "stmt.h"
#include "environment.h"

#ifndef INTERPRETER_H_
#define INTERPRETER_H_

class Interpreter {
public:
  Interpreter() = default;
  void interpret(const std::vector<Stmt *> statements);

private:
  Environment environment;
};

#endif // INTERPRETER_H_
