#include "environment.h"
#include "token.h"

#ifndef EXPR_H_
#define EXPR_H_

// expression -> assignment;
// assignment -> IDENTIFIER "=" assignment | | logic_or ;
// logic_or -> logic_and ( "or" logic_and )* ;
// logic_and ->  equality ( "and" equality )* ;
// equality -> comparison ( ( "!=" | "==" ) comparison )* ;
// comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term -> factor  ( ( "-" | "+" ) factor )* ;
// unary -> ("!" | "-" ) unary | primary ;
// primary ->? NUMBER | STRING | BOOL | NIL | "(" expression ")" | IDENTIFIER;

struct Expr {
  virtual ~Expr() = default;
  virtual std::string print() const = 0;
  virtual LiteralType evaluate(Environment *environment) = 0;
};

struct Literal : public Expr {
  LiteralType value;

  Literal(LiteralType value) : value(value) {}
  std::string print() const override;
  LiteralType evaluate(Environment *environment) override;
};

struct Variable : public Expr {
  Token name;

  Variable(const Token &token) : name(token) {}
  std::string print() const override;
  LiteralType evaluate(Environment *environment) override;
};

struct Grouping : public Expr {
  Expr *expr;

  Grouping(Expr *expr) : expr(expr) {}
  std::string print() const override;
  LiteralType evaluate(Environment *environment) override;
};

struct Unary : public Expr {
  Token op;
  Expr *right;

  Unary(Token op, Expr *right) : op(op), right(right) {}
  std::string print() const override;
  LiteralType evaluate(Environment *environment) override;
};

struct Binary : public Expr {
  Expr *left;
  Token op;
  Expr *right;

  Binary(Expr *left, Token op, Expr *right)
      : left(left), op(op), right(right) {}
  std::string print() const override;
  LiteralType evaluate(Environment *environment) override;
};

struct Assign : public Expr {
  Variable* target;
  Expr *value;

  Assign(Variable *target, Expr *value) : target(target), value(value) {}
  std::string print() const override;
  LiteralType evaluate(Environment *environment) override;
};

struct Logical : public Expr {
  Expr *left;
  Token op;
  Expr *right;

  Logical(Expr *left, const Token &token, Expr *right)
      : left(left), op(token), right(right) {}
  std::string print() const override;
  LiteralType evaluate(Environment *environment) override;
};

void delete_expr(Expr *expr);

#endif // EXPR_H_
