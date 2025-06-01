#ifndef STMT_H_
#define STMT_H_

#include "environment.h"
#include "expr.h"
#include "token.h"
#include <vector>

class Interpreter;
class Environment;

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

struct Stmt {
  virtual ~Stmt() = default;
  virtual void execute(Interpreter *interpreter) = 0;
};

struct VarDecl : public Stmt {
  Token name;
  Expr *initializer;

  VarDecl(const Token &token, Expr *expr) : name(token), initializer(expr) {}
  void execute(Interpreter *interpreter) override;
};

struct ExprStmt : public Stmt {
  Expr *expr;

  ExprStmt(Expr *expr) : expr(expr) {}
  void execute(Interpreter *interpreter) override;
};

struct PrintStmt : public Stmt {
  Expr *expr;

  PrintStmt(Expr *expr) : expr(expr) {}
  void execute(Interpreter *interpreter) override;
};

struct Block : public Stmt {
  std::vector<Stmt *> statements;

  Block(const std::vector<Stmt *> &statements)
      : statements(std::move(statements)) {}
  void execute(Interpreter *interpreter) override;
};

struct IfStmt : public Stmt {
  Expr *condition;
  Stmt *then_branch;
  Stmt *else_branch;

  IfStmt(Expr *cond, Stmt *then, Stmt *else_)
      : condition(cond), then_branch(then), else_branch(else_) {}
  void execute(Interpreter *interpreter) override;
};

struct WhileStmt : public Stmt {
  Expr *condition;
  Stmt *body;

  WhileStmt(Expr *cond, Stmt *body) : condition(cond), body(body) {}
  void execute(Interpreter *interpreter) override;
};

struct FuncStmt : public Stmt {
  Token name;
  std::vector<Token> params;
  std::vector<Stmt *> body;

  FuncStmt(const Token &name, const std::vector<Token> &params,
           const std::vector<Stmt *> &body)
      : name(name), params(std::move(params)), body(std::move(body)) {}
  void execute(Interpreter *interpreter) override;
};

struct ReturnStmt : public Stmt {
  Token keyword;
  Expr* value;

  ReturnStmt(const Token& keyword, Expr* value) : keyword(keyword), value(value) {}
  void execute(Interpreter* interpreter) override;
};

void execute_block(const std::vector<Stmt *> &statements, Environment *env,
                   Interpreter *interpreter);
void delete_stmt(Stmt *stmt);

#endif // STMT_H_
