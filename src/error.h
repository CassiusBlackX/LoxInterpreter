#include <sstream>
#include <stdexcept>
#include <string>

#include "token.h"

#ifndef ERROR_H_
#define ERROR_H_

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

void handle_runtime_error(const RuntimeError &e);

#endif // ERROR_H_
