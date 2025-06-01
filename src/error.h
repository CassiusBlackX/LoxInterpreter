#ifndef ERROR_H_
#define ERROR_H_

#include <sstream>
#include <stdexcept>
#include <string>

#include "token.h"

void error(std::size_t, const std::string &);

class RuntimeError : public std::runtime_error {
public:
  RuntimeError(const Token &token, const std::string &message)
      : std::runtime_error(message), token(token) {}

  const char *what() const noexcept override {
    std::ostringstream oss;
    static std::string formatted_str;
    oss << std::runtime_error::what() << "\n[line " << token.get_line() << "]";
    formatted_str = oss.str();
    return formatted_str.c_str();
  }

private:
  Token token;
};

struct ReturnException : RuntimeError {
  Object value;
  ReturnException(const Object &value)
      : value(value),
        RuntimeError(Token(TokenType::Return, "return", 0), "return") {}
  const char *what() {
    static std::string formatted_str;
    formatted_str = value.to_string();
    return formatted_str.c_str();
  }
};

void handle_runtime_error(const RuntimeError &e);

#endif // ERROR_H_
