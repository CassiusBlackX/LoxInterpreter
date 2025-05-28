#include "stmt.h"
#include "error.h"
#include <iostream>
#include <vector>

// program -> declaration* EOF ;
// declaration -> varDecl | statement ;
// varDecl -> "var" IDENTIFIER ( "=" expression )? ";" ;
// statement -> exprStmt | ifStmt | whileStmt | forStmt | printStmt | block;
// block -> "{" declaration* "}" ;
// exprStmt -> expression ";" ;
// printStmt -> "print" expression ";" ;
// ifStmt -> "if" "(" expression ")" statement ( "else" statement )? ;
// whileStmt -> "while" "(" expression ")" statement ;
// forStmt -> "for" "(" ( varDecl | exprStmt | ";")
//             expression? ";"
//             expression? ")" statement ;

void ExprStmt::execute(Environment *_environment) {
  expr->evaluate(_environment);
}

void PrintStmt::execute(Environment *_environment) {
  Object value = expr->evaluate(_environment);
  std::cout << value << std::endl;
}

void VarDecl::execute(Environment *environment) {
  Object value = Object(); // if the value does not have an
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

void IfStmt::execute(Environment *environment) {
  Object cond_bool = condition->evaluate(environment);
  // using bool() cast to check if cond is true or false
  if (static_cast<bool>(cond_bool)) {
    then_branch->execute(environment);
  } else if (else_branch != nullptr) {
    // only when condition == false && got an 'else' statement, will we execute
    // else statement avoid dangling pointer!
    else_branch->execute(environment);
  }
}

void WhileStmt::execute(Environment *environment) {
  while (static_cast<bool>(condition->evaluate(environment))) {
    body->execute(environment);
  }
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
