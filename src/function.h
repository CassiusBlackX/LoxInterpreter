#ifndef FUNCTION_H_
#define FUNCTION_H_

#include "callable.h"
#include "environment.h"
#include "interpreter.h"
#include "stmt.h"
#include <cstddef>

struct Function : public Callable {
  FuncStmt *declaration;
  bool is_initializer;
  Environment *closure;

  Function(FuncStmt *func_decl, bool is_initializer, Environment *closure)
      : declaration(func_decl), is_initializer(is_initializer),
        closure(closure) {}

  Object call(Interpreter *interpreter,
              const std::vector<Object> &arguments) override;
  size_t arity() const override;
  std::string to_string() const override;
};


#endif // FUNCTION_H_
