#include "scanner.h"
#include "error.h"
#include <cmath>
#include <stdexcept>
#include <string_view>
#include <vector>

static bool is_digit(char c) { return '0' <= c && c <= '9'; }
static bool is_alphabet(char c) {
  return ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z');
}
static bool is_alphanumeric(char c) { return is_digit(c) || is_alphabet(c); }

char Scanner::advance() { return source[current++]; }

bool Scanner::next(char expected) {
  if (at_end())
    return false;
  if (source[current] != expected)
    return false;
  current++;
  return true;
}

char Scanner::peek() const {
  if (at_end())
    return '\0';
  return source[current];
}

char Scanner::peek_next() const {
  if (current + 1 >= source.length())
    return '\0';
  return source[current + 1];
}

void Scanner::add_token(TokenType type) {
  std::string value(source.substr(start, current - start));
  tokens.emplace_back(type, value, line);
}

void Scanner::handle_string() {
  while (peek() != '"' && !at_end()) {
    if (peek() == '\n') {
      line++;
    }
    advance();
  }
  if (at_end()) {
    error(line, "Unterminated string!");
  }
  advance(); // consume the closing '"'
  // TODO: delete the following line
  // std::string_view value = source.substr(start + 1, current - start - 2);
  // the `value` created above is useless, because the token's LiteralType's
  // String will be a reference of its lexeme instead of the `value` above
  add_token(TokenType::String);
}

void Scanner::handle_number() {
  while (is_digit(peek())) {
    advance();
  }
  if (peek() == '.' && is_digit(peek_next())) {
    advance(); // consume '.'
    while (is_digit(peek())) {
      advance();
    }
  }
  // double value = stod(source.substr(start, current));
  add_token(TokenType::Number);
}

void Scanner::handle_identifier() {
  while (is_alphanumeric(peek())) {
    advance();
  }
  std::string_view value = source.substr(start, current - start);
  TokenType type = match_keyword(value);
  add_token(type);
}

void Scanner::scan_token() {
  char c = advance();
  switch (c) {
  case '(':
    add_token(TokenType::LeftParen);
    break;
  case ')':
    add_token(TokenType::RightParen);
    break;
  case '{':
    add_token(TokenType::LeftBrace);
    break;
  case '}':
    add_token(TokenType::RightBrace);
    break;
  case ',':
    add_token(TokenType::Comma);
    break;
  case '.':
    add_token(TokenType::Dot);
    break;
  case ';':
    add_token(TokenType::SemiColon);
    break;
  case '+':
    add_token(TokenType::Plus);
    break;
  case '-':
    add_token(TokenType::Minus);
    break;
  case '*':
    add_token(TokenType::Star);
    break;
  case '!':
    add_token(next('=') ? TokenType::BangEqual : TokenType::Bang);
    break;
  case '=':
    add_token(next('=') ? TokenType::EqualEqual : TokenType::Equal);
    break;
  case '<':
    add_token(next('=') ? TokenType::LessEqual : TokenType::Less);
    break;
  case '>':
    add_token(next('=') ? TokenType::GreaterEqual : TokenType::Greater);
    break;
  case '/':
    if (next('/')) {
      while (peek() != '\n' && !at_end())
        advance();
    } else {
      add_token(TokenType::Slash);
    }
    break;
  case '\n':
    line++;
  case ' ':
  case '\r':
  case '\t':
    break;
  case '"':
    handle_string();
    break;
  default:
    if (is_digit(c)) {
      handle_number();
    } else if (is_alphabet(c) || c == '_') {
      handle_identifier();
    } else {
      std::string message = "unexpected character: ";
      message.push_back(c);
      error(line, message);
    }
  }
}

std::vector<Token> Scanner::scan_tokens() {
  while (!at_end()) {
    start = current;
    scan_token();
  }
  tokens.emplace_back(TokenType::Eof, "", line);
  return tokens;
}
