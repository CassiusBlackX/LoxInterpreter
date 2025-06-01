// program -> declaration* EOF ;
// declaration -> funcDecl | varDecl | statement ;
// funcDecl -> "fun" function ;
// function -> IDENTIFIER "(" parameters? ")" block;
// parameters -> IDENTIFIER ("," IDENTIFIER )* ;
// varDecl -> "var" IDENTIFIER ( "=" expression )? ";" ;
// statement -> exprStmt | ifStmt | whileStmt | forStmt | printStmt | returnStmt
// | block; block -> "{" declaration* "}" ; exprStmt -> expression ";" ;
// printStmt -> "print" expression ";" ;
// ifStmt -> "if" "(" expression ")" statement ( "else" statement )? ;
// whileStmt -> "while" "(" expression ")" statement ;
// forStmt -> "for" "(" ( varDecl | exprStmt | ";")
//             expression? ";"
//             expression? ")" statement ;
//  returnStmt -> "return" expression? ";" ;
//
// expression -> assignment;
// assignment -> IDENTIFIER "=" assignment | | logic_or ;
// logic_or -> logic_and ( "or" logic_and )* ;
// logic_and ->  equality ( "and" equality )* ;
// equality -> comparison ( ( "!=" | "==" ) comparison )* ;
// comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term -> factor  ( ( "-" | "+" ) factor )* ;
// unary -> ("!" | "-" ) unary | call;
// call -> primary ( "(" arguments? ")" )* ;
// arguments -> expression ( "," expression )* ;
// primary ->? NUMBER | STRING | BOOL | NIL | "(" expression ")" | IDENTIFIER;

#ifndef PARSER_H_
#define PARSER_H_

#include <cstddef>
#include <initializer_list>
#include <stdexcept>
#include <string_view>
#include <vector>

#include "expr.h"
#include "stmt.h"
#include "token.h"

class Parser {
public:
  Parser(const std::vector<Token> &tokens) : tokens(std::move(tokens)) {}
  std::vector<Stmt *> parse();

private:
  class ParseError : public std::runtime_error {
  public:
    explicit ParseError(const Token &token, const std::string &message)
        : std::runtime_error(message), token(token) {}

  private:
    Token token;
  };

private:
  Stmt *declaration();
  Stmt *function(const std::string &func_type);
  Stmt *var_declaration();
  Stmt *statement();
  std::vector<Stmt *> block();
  Stmt *return_statement();
  Stmt *if_statement();
  Stmt *while_statement();
  Stmt *for_statement();
  Stmt *print_statement();
  Stmt *expression_statement();
  Expr *expression();
  Expr *assignment();
  Expr *logic_or();
  Expr *logic_and();
  Expr *equality();
  Expr *comparison();
  Expr *term();
  Expr *factor();
  Expr *unary();
  Expr *call();
  Expr *finish_call(Expr *callee);
  Expr *primary();
  Token advance();
  bool match(std::initializer_list<TokenType> types);
  bool check(const TokenType type) const;
  bool at_end() const;
  Token peek() const;
  Token previous() const;
  Token consume(const TokenType expected_type, std::string_view message);
  ParseError error(const Token &token, std::string_view message) const;
  void synchronize();

private:
  std::vector<Token> tokens;
  size_t current = 0;
};

#endif // PARSER_H_
