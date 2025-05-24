#include <cstddef>
#include <string>
#include <vector>

#include "token.h"

#ifndef SCANNER_H_
#define SCANNER_H_

class Scanner {
public:
  Scanner() = default;
  Scanner(const std::string &content) : source(content) {}
  std::vector<Token> scan_tokens();

private:
  void scan_token();
  bool at_end() const { return current >= source.length(); }
  char advance();
  bool next(char c);
  char peek() const;
  char peek_next() const;
  void add_token(TokenType, LiteralType);
  void add_token(TokenType);
  void handle_string();
  void handle_number();
  void handle_identifier();

private:
  std::string_view source;
  size_t start = 0;
  size_t current = 0;
  size_t line = 1;
  std::vector<Token> tokens;
};

#endif // SCANNER_H_
