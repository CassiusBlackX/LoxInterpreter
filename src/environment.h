#include <string>
#include <unordered_map>

#include "token.h"

#ifndef ENVIRONMENT_H_
#define ENVIRONMENT_H_

class Environment {
public:
  Environment() : enclosing(nullptr) {}
  Environment(Environment* enclosing) : enclosing(enclosing) {}
  void define(const std::string &name, const LiteralType &value) {
    values.insert({name, value});
  }

  LiteralType get(const Token &name) const;
  void assign(const Token &tname, const LiteralType &value);

private:
  // using string as key instead of Token
  // 1. do not need to implement hash(Token)
  // 2. for all Token in difference place of the code, but with the same name,
  // is the same variable
  std::unordered_map<std::string, LiteralType> values;
  Environment *enclosing;
};

#endif // ENVIRONMENT_H_
