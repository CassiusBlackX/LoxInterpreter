#include "stmt.h"
#include "error.h"
#include <iostream>
#include <vector>

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
  Environment block_environment = Environment(environment);
  execute_block(statements, &block_environment);
}

void delete_stmt(Stmt *stmt) {
  if (stmt == nullptr)
    return;

  if (auto var_decl = dynamic_cast<VarDecl *>(stmt)) {
    delete_expr(var_decl->initializer);
    var_decl->initializer = nullptr;
  } else if (auto expr_stmt = dynamic_cast<ExprStmt *>(stmt)) {
    delete_expr(expr_stmt->expr);
    expr_stmt->expr = nullptr;
  } else if (auto print_stmt = dynamic_cast<PrintStmt *>(stmt)) {
    delete_expr(print_stmt->expr);
    print_stmt->expr = nullptr;
  } else if (auto block_stmt = dynamic_cast<Block *>(stmt)) {
    for (Stmt *stat : block_stmt->statements) {
      delete_stmt(stat);
      stat = nullptr;
    }
  }
  delete stmt;
  stmt = nullptr;
}
