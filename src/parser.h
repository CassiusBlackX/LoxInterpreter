#include <cstddef>
#include <initializer_list>
#include <string_view>
#include <vector>

#include "expr.h"
#include "token.h"

// expression -> equality ;
// equality -> comparison ( ( "!=" | "==" ) comparison )* ;
// comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term -> factor  ( ( "-" | "+" ) factor )* ;
// unary -> ("!" | "-" ) unary | primary ;
// primary ->? NUMBER | STRING | BOOL | NIL | "(" expression ")" ;

#ifndef PARSER_H_
#define PARSER_H_

class Parser {
public:
  Parser(const std::vector<Token> &tokens) : tokens(std::move(tokens)) {}
  Expr *parse();

private:
  Expr *expression();
  Expr *equality();
  Expr *comparison();
  Expr *term();
  Expr *factor();
  Expr *unary();
  Expr *primary();
  void advance();
  bool match(std::initializer_list<TokenType> types);
  bool check(TokenType type) const;
  bool at_end() const ;
  Token peek() const;
  Token previous() const;
  Token consume(TokenType expected_type, std::string_view message);
  void error(Token token, std::string_view message) const;
  void synchronize();

private:
  std::vector<Token> tokens;
  size_t current;
};

#endif // PARSER_H_
