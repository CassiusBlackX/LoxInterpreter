#include "environment.h"
#include "error.h"
#include <cstddef>

Object Environment::get(const Token &name) const {
  if (values.contains(name.get_lexeme())) {
    return values.at(name.get_lexeme());
  }

  if (enclosing != nullptr)
    return enclosing->get(name);

  throw RuntimeError(name, std::string("Undefined variable '") +
                               name.get_lexeme() + "'.");
}

Object Environment::get_at(size_t distance, const std::string &name) {
  return ancestor(distance)->values.at(name);
}

void Environment::assign(const Token &name, const Object &value) {
  if (values.contains(name.get_lexeme())) {
    values[name.get_lexeme()] = value;
    return;
  }

  if (enclosing != nullptr) {
    enclosing->assign(name, value);
    return;
  }

  throw RuntimeError(name, "Undefined variable '" + name.get_lexeme() + "'.");
}

void Environment::assign(size_t distance, const Token &name, const Object &value) {
  return ancestor(distance)->assign(name, value);
}

Environment *Environment::ancestor(size_t distance) {
  Environment *env = this;
  for (size_t i = 0; i < distance; i++) {
    env = env->enclosing;
  }
  return env;
}
