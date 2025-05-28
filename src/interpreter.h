#ifndef INTERPRETER_H_
#define INTERPRETER_H_

#include <vector>

#include "environment.h"
#include "stmt.h"

class Interpreter {
public:
  Interpreter() ;
  void interpret(const std::vector<Stmt *> statements);

  void enter_scope();
  void exit_scope();
  Environment* get_current();
  Environment* get_global();

private:
  Environment environment;
  Environment globals;
};

#endif // INTERPRETER_H_
