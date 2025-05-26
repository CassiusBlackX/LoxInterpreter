#include "stmt.h"
#include "error.h"
#include <iostream>

void ExprStmt::execute(Environment *_environment) {
  expr->evaluate(_environment);
}

void PrintStmt::execute(Environment *_environment) {
  LiteralType value = expr->evaluate(_environment);
  std::cout << value << std::endl;
}

void VarDecl::execute(Environment *environment) {
  LiteralType value = LiteralType(); // if the value does not have an
                                     // initializer, its value will be null
  if (initializer != nullptr) {
    value = initializer->evaluate(environment);
  }

  environment->define(name.get_lexeme(), value);
}


static void execute_block(const std::vector<Stmt *> &statements,
                          Environment *environment) {
  try {
    for (Stmt *statement : statements) {
      statement->execute(environment);
    }
  } catch (const RuntimeError &e) {
    std::cerr << e.what() << std::endl;
  }
}

void Block::execute(Environment *environment) {
  Environment *block_environment = new Environment(environment);
  execute_block(statements, block_environment);
}
