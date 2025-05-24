#include "token.h"

#ifndef EXPR_H_
#define EXPR_H_

struct Expr {
  virtual ~Expr() = default;
  virtual std::string print() = 0;
};

struct Literal : public Expr {
  LiteralType value;

  Literal(LiteralType value) : value(value) {}
  std::string print() override;
};

struct Grouping : public Expr {
  Expr *expr;

  Grouping(Expr *expr) : expr(expr) {}
  std::string print() override;
};

struct Unary : public Expr {
  Token op;
  Expr *right;

  Unary(Token op, Expr *right) : op(op), right(right) {}
  std::string print() override;
};

struct Binary : public Expr {
  Expr *left;
  Token op;
  Expr *right;

  Binary(Expr *left, Token op, Expr *right)
      : left(left), op(op), right(right) {}
  std::string print() override;
};

void delete_expr(Expr *expr);
#endif // EXPR_H_
