#ifndef EXPR_H_
#define EXPR_H_

#include <vector>

#include "callable.h"
#include "token.h"

class Interpreter;

struct Expr {
  virtual ~Expr() = default;
  virtual std::string to_string() const = 0;
  virtual Object evaluate(Interpreter *interpreter) = 0;
};

struct Literal : public Expr {
  Object value;

  Literal(Object value) : value(value) {}
  std::string to_string() const override;
  Object evaluate(Interpreter *interpreter) override;
};

struct Variable : public Expr {
  Token name;

  Variable(const Token &token) : name(token) {}
  std::string to_string() const override;
  Object evaluate(Interpreter *interpreter) override;
};

struct Grouping : public Expr {
  Expr *expr;

  Grouping(Expr *expr) : expr(expr) {}
  std::string to_string() const override;
  Object evaluate(Interpreter *interpreter) override;
};

struct Unary : public Expr {
  Token op;
  Expr *right;

  Unary(Token op, Expr *right) : op(op), right(right) {}
  std::string to_string() const override;
  Object evaluate(Interpreter *interpreter) override;
};

struct Binary : public Expr {
  Expr *left;
  Token op;
  Expr *right;

  Binary(Expr *left, Token op, Expr *right)
      : left(left), op(op), right(right) {}
  std::string to_string() const override;
  Object evaluate(Interpreter *interpreter) override;
};

struct Assign : public Expr {
  Variable *target;
  Expr *value;

  Assign(Variable *target, Expr *value) : target(target), value(value) {}
  std::string to_string() const override;
  Object evaluate(Interpreter *interpreter) override;
};

struct Logical : public Expr {
  Expr *left;
  Token op;
  Expr *right;

  Logical(Expr *left, const Token &token, Expr *right)
      : left(left), op(token), right(right) {}
  std::string to_string() const override;
  Object evaluate(Interpreter *interpreter) override;
};

struct Call : public Expr{
  Expr *callee;
  Token paren;
  std::vector<Expr *> arguments;

  Call(Expr *callee, const Token &paren, const std::vector<Expr *> &args)
      : callee(callee), paren(paren), arguments(std::move(args)) {}
  std::string to_string() const override;
  Object evaluate(Interpreter *interpreter) override;
};

void delete_expr(Expr *expr);

#endif // EXPR_H_
