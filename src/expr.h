#include "token.h"

#ifndef EXPR_H_
#define EXPR_H_

struct Expr {
  virtual void interpret();
};

struct Literal : public Expr {
  LiteralType value;

  Literal(LiteralType value) : value(value) {}
};

struct Grouping : public Expr {
  Expr* expr;
  
  Grouping(Expr* expr) : expr(expr) {}
};

struct Unary : public Expr {
  Token op;
  Expr* right;

  Unary(Token op, Expr* right) : op(op), right(right) {} 
};

struct Binary : public Expr {
  Expr* left;
  Token op;
  Expr* right;

  Binary(Expr* left, Token op, Expr* right) : left(left), op(op), right(right) {}
};

#endif // EXPR_H_
