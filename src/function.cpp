#include "function.h"
#include "environment.h"
#include "error.h"
#include "interpreter.h"
#include "object.h"
#include "stmt.h"
#include <cstddef>

Object Function::call(Interpreter *interpreter,
                      const std::vector<Object> &arguments) {
  Environment environment(interpreter->get_global());
  for (size_t i = 0; i < declaration->params.size(); i++) {
    environment.define(declaration->params.at(i).get_lexeme(), arguments.at(i));
  }
  try {
    execute_block(declaration->body, &environment, interpreter);
  } catch (ReturnException return_value) {
    if (is_initializer) {
      return closure->get_at(0, "this");
    }
    return return_value.value;
  }
  if (is_initializer) {
    return closure->get_at(0, "this");
  } else {
    return Object();
  }
}

size_t Function::arity() const { return declaration->params.size(); }

std::string Function::to_string() const {
  return std::string("<fn ") + declaration->name.get_lexeme() + ">";
}
