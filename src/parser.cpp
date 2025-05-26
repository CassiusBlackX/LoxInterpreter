#include "parser.h"
#include "error.h"
#include <iostream>
#include <stdexcept>
#include <string>

// expression -> equality ;
// equality -> comparison ( ( "!=" | "==" ) comparison )* ;
// comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term -> factor  ( ( "-" | "+" ) factor )* ;
// unary -> ("!" | "-" ) unary | primary ;
// primary ->? NUMBER | STRING | BOOL | NIL | "(" expression ")" ;

static void token_error(Token token, std::string_view message) {
  if (token.get_tokentype() == TokenType::Eof) {
    error(token.get_line(), std::string("at end ").append(message));
  } else {
    error(token.get_line(), std::string("at '") +
                                std::string(token.get_lexeme()) + "'. " +
                                std::string(message));
  }
}

Stmt *Parser::declaration() {
  try {
    if (match({TokenType::Var}))
      return var_declaration();
    return statement();
  } catch (const ParseError &e) {
    synchronize();
    return nullptr;
  }
}

Stmt *Parser::var_declaration() {
  Token name = consume(TokenType::Identifier, "Expect variable name");

  Expr *initializer = nullptr;
  if (match({TokenType::Equal})) {
    initializer = expression();
  }
  consume(TokenType::SemiColon, "Expect ';' after variable declaration");
  return new VarDecl(name, initializer);
}

Stmt *Parser::statement() {
  if (match({TokenType::Print})) {
    return print_statement();
  }
  if (match({TokenType::LeftBrace})) {
    return new Block(block());
  }
  return expression_statement();
}

std::vector<Stmt *> Parser::block() {
  std::vector<Stmt *> statements;
  while (!check(TokenType::RightBrace) && !at_end()) {
    statements.push_back(declaration());
  }
  consume(TokenType::RightBrace, "Expect '}' after block");
  return statements;
}

Stmt *Parser::print_statement() {
  Expr *value = expression();
  consume(TokenType::SemiColon, "Expect ';' after value.");
  return new PrintStmt(value);
}

Stmt *Parser::expression_statement() {
  Expr *expr = expression();
  consume(TokenType::SemiColon, "Expect ';' after expression");
  return new ExprStmt(expr);
}

Expr *Parser::expression() { return assignment(); }

Expr *Parser::assignment() {
  Expr *expr = equality();

  if (match({TokenType::Equal})) {
    Token equal = previous();
    Expr *value = assignment();

    if (auto var = dynamic_cast<Variable *>(expr)) {
      Token name = var->name;
      return new Assign(name, value);
    }
    error(equal, "Invalid assignment target");
  }
  return expr;
}

Expr *Parser::equality() {
  Expr *expr = comparison();

  while (match({TokenType::BangEqual, TokenType::Equal})) {
    Token op = previous();
    Expr *right = comparison();
    expr = new Binary(expr, op, right);
  }
  return expr;
}

Expr *Parser::comparison() {
  Expr *expr = term();

  while (match({TokenType::Greater, TokenType::GreaterEqual, TokenType::Less,
                TokenType::LessEqual})) {
    Token op = previous();
    Expr *right = term();
    expr = new Binary(expr, op, right);
  }
  return expr;
}

Expr *Parser::term() {
  Expr *expr = factor();

  while (match({TokenType::Minus, TokenType::Plus})) {
    Token op = previous();
    Expr *right = factor();
    expr = new Binary(expr, op, right);
  }
  return expr;
}

Expr *Parser::factor() {
  Expr *expr = unary();

  while (match({TokenType::Slash, TokenType::Star})) {
    Token op = previous();
    Expr *right = unary();
    expr = new Binary(expr, op, right);
  }
  return expr;
}

Expr *Parser::unary() {
  if (match({TokenType::Bang, TokenType::Minus})) {
    Token op = previous();
    Expr *right = unary();
    return new Unary(op, right);
  }
  return primary();
}

Expr *Parser::primary() {
  if (match({TokenType::False})) {
    return new Literal(LiteralType(false));
  } else if (match({TokenType::True})) {
    return new Literal(LiteralType(true));
  } else if (match({TokenType::Nil})) {
    return new Literal(LiteralType());
  } else if (match({TokenType::Number, TokenType::String})) {
    return new Literal(previous().get_literal());
  } else if (match({TokenType::Identifier})) {
    return new Variable(previous());
  } else if (match({TokenType::LeftParen})) {
    Expr *expr = expression();
    Token _token =
        consume(TokenType::RightParen, "Expected ')' after expression");
    return new Grouping(expr);
  } else {
    error(peek(), "expected expression");
    return nullptr;
  }
}

// an "or" match
// any of the element given mathed, return true
// no match in the given, return false
bool Parser::match(std::initializer_list<TokenType> types) {
  for (auto type : types) {
    if (check(type)) {
      advance();
      return true;
    }
  }
  return false;
}

bool Parser::check(const TokenType type) const {
  if (at_end())
    return false;
  else
    return peek().get_tokentype() == type;
}

Token Parser::advance() {
  if (!at_end()) {
    current++;
  }
  return previous();
}

bool Parser::at_end() const { return peek().get_tokentype() == TokenType::Eof; }

Token Parser::peek() const { return tokens.at(current); }

Token Parser::previous() const { return tokens.at(current - 1); }

Token Parser::consume(const TokenType type, std::string_view message) {
  if (check(type))
    return advance();
  throw error(peek(), message);
}

Parser::ParseError Parser::error(const Token &token,
                                 std::string_view message) const {
  token_error(token, message);
  return ParseError(token, std::string(message));
}

void Parser::synchronize() {
  advance();

  while (!at_end()) {
    if (previous().get_tokentype() == TokenType::SemiColon)
      return;

    switch (peek().get_tokentype()) {
    case TokenType::Class_:
    case TokenType::Fun:
    case TokenType::Var:
    case TokenType::For:
    case TokenType::If:
    case TokenType::While:
    case TokenType::Print:
    case TokenType::Return:
      return;
    default:
      // do nothing
      continue;
    }
    advance();
  }
}

std::vector<Stmt *> Parser::parse() {
  std::vector<Stmt *> statements;
  while (!at_end()) {
    statements.push_back(declaration());
  }
  return statements;
}
