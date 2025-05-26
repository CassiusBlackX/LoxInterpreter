#include "environment.h"
#include "expr.h"
#include <vector>

#ifndef STMT_H_
#define STMT_H_

// program -> declaration* EOF ;
// declaration -> varDecl | statement ;
// varDecl -> "var" IDENTIFIER ( "=" expression )? ";" ;
// statement -> exprStmt | printStmt | block;
// block -> "{" declaration* "}" ;
// exprStmt -> expression ";" ;
// printStmt -> "print" expression ";" ;

struct Stmt {
  virtual ~Stmt() = default;
  virtual void execute(Environment *environment) = 0;
};

struct VarDecl : public Stmt {
  Token name;
  Expr *initializer;

  VarDecl(const Token &token, Expr *expr) : name(token), initializer(expr) {}
  void execute(Environment *environment) override;
};

struct ExprStmt : public Stmt {
  Expr *expr;

  ExprStmt(Expr *expr) : expr(expr) {}
  void execute(Environment *environment) override;
};

struct PrintStmt : public Stmt {
  Expr *expr;

  PrintStmt(Expr *expr) : expr(expr) {}
  void execute(Environment *environment) override;
};

struct Block : public Stmt {
  std::vector<Stmt *> statements;

  Block(const std::vector<Stmt *> &statements)
      : statements(std::move(statements)) {}
  void execute(Environment *environment) override;
};

void delete_stmt(Stmt* stmt);

#endif // STMT_H_
