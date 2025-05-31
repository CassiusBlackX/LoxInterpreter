#ifndef CALLABLE_H_
#define CALLABLE_H_

#include <cstddef>
#include <string>
#include <vector>

#include "object.h"

class Interpreter;

class Callable {
public:
  virtual Object call(Interpreter *interpreter,
                      const std::vector<Object> &arguments) = 0;
  virtual size_t arity() const = 0;
  virtual std::string to_string() const = 0;
};

#endif // CALLABLE_H_
