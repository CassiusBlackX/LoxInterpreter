#include "parser.h"
#include "error.h"
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

Expr *Parser::expression() { return equality(); }

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

bool Parser::check(TokenType type) const {
  if (at_end())
    return false;
  else
    return peek().get_tokentype() == type;
}

void Parser::advance() {
  if (!at_end()) {
    current++;
  }
  previous();
}

bool Parser::at_end() const { return peek().get_tokentype() == TokenType::Eof; }

Token Parser::peek() const { return tokens[current]; }

Token Parser::previous() const { return tokens[current - 1]; }

Token Parser::consume(TokenType type, std::string_view message) {
  if (!check(type)) {
    error(previous(), message);
    // TODO: throw anything here?
    throw std::invalid_argument("expecting ')' at the end of an expression!");
  }
  advance();
  return previous();
}

void Parser::error(Token token, std::string_view message) const {
  token_error(token, message);
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

Expr *Parser::parse() {
  try {
    return expression();
  } catch (...) {
    return nullptr;
  }
}
