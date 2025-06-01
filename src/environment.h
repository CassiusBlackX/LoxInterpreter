#ifndef ENVIRONMENT_H_
#define ENVIRONMENT_H_

#include <cstddef>
#include <string>
#include <unordered_map>

#include "token.h"

class Environment {
public:
  Environment() : enclosing(nullptr) {}
  ~Environment();
  Environment(Environment *enclosing) : enclosing(enclosing) {}
  void define(const std::string &name, const Object &value) {
    values.insert({name, value});
  }

  Object get(const Token &name) const;
  Object get_at(size_t distance, const std::string& name) ;
  void assign(const Token &tname, const Object &value);
  void assign(size_t distance, const Token &name, const Object &value);

private:
  Environment* ancestor(size_t distance) ;

private:
  // using string as key instead of Token
  // 1. do not need to implement hash(Token)
  // 2. for all Token in difference place of the code, but with the same name,
  // is the same variable
  std::unordered_map<std::string, Object> values;
  Environment *enclosing;
};

#endif // ENVIRONMENT_H_
