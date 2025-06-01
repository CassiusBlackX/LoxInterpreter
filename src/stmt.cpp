#include "stmt.h"
#include "environment.h"
#include "error.h"
#include "function.h"
#include "interpreter.h"
#include "object.h"
#include <iostream>
#include <vector>

void ExprStmt::execute(Interpreter *interpreter) {
  expr->evaluate(interpreter);
}

void PrintStmt::execute(Interpreter *interpreter) {
  Object value = expr->evaluate(interpreter);
  std::cout << value << std::endl;
}

void VarDecl::execute(Interpreter *interpreter) {
  Object value = Object(); // if the value does not have an
                           // initializer, its value will be null
  if (initializer != nullptr) {
    value = initializer->evaluate(interpreter);
  }

  interpreter->get_current()->define(name.get_lexeme(), value);
}

void execute_block(const std::vector<Stmt *> &statements, Environment *env,
                   Interpreter *interpreter) {
  Environment *previous = interpreter->get_current();
  try {
    *interpreter->set_current() = env;
    for (Stmt *statement : statements) {
      statement->execute(interpreter);
    }
  } catch (RuntimeError &e) {
    *interpreter->set_current() = previous;
    if (dynamic_cast<ReturnException*>(&e)) {
      throw;
    }
  }
  *interpreter->set_current() = previous;
}

void Block::execute(Interpreter *interpreter) {
  Environment block_environment = Environment(interpreter->get_current());
  execute_block(statements, &block_environment, interpreter);
}

void IfStmt::execute(Interpreter *interpreter) {
  Object cond_bool = condition->evaluate(interpreter);
  // using bool() cast to check if cond is true or false
  if (static_cast<bool>(cond_bool)) {
    then_branch->execute(interpreter);
  } else if (else_branch != nullptr) {
    // only when condition == false && got an 'else' statement, will we execute
    // else statement avoid dangling pointer!
    else_branch->execute(interpreter);
  }
}

void WhileStmt::execute(Interpreter *interpreter) {
  while (static_cast<bool>(condition->evaluate(interpreter))) {
    body->execute(interpreter);
  }
}

void FuncStmt::execute(Interpreter *interpreter) {
  // FIXME: a memory leak here!
  // how do we free this space?
  Function *function = new Function(this, false, interpreter->get_current());
  interpreter->get_current()->define(name.get_lexeme(), function);
}

void ReturnStmt::execute(Interpreter *interpreter) {
  Object return_value = Object();
  if (value != nullptr) {
    return_value = value->evaluate(interpreter);
  }
  throw ReturnException(return_value);
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
  } else if (auto if_stmt = dynamic_cast<IfStmt *>(stmt)) {
    delete_expr(if_stmt->condition);
    if_stmt->condition = nullptr;
    delete_stmt(if_stmt->then_branch);
    if_stmt->then_branch = nullptr;
    delete_stmt(if_stmt->else_branch);
    if_stmt->else_branch = nullptr;
  } else if (auto while_stmt = dynamic_cast<WhileStmt *>(stmt)) {
    delete_expr(while_stmt->condition);
    while_stmt->condition = nullptr;
    delete_stmt(while_stmt->body);
    while_stmt->body = nullptr;
  }
  delete stmt;
  stmt = nullptr;
}
