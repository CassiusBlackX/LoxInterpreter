#include "interpreter.h"
#include "callable.h"
#include "error.h"
#include "expr.h"

#include <chrono>
#include <cstddef>
#include <vector>

struct ClockCallable : public Callable {
  Object call(Environment *env, const std::vector<Object> &args) override {
    auto now = std::chrono::system_clock::now();
    auto duration = now.time_since_epoch();
    auto millis =
        std::chrono::duration_cast<std::chrono::milliseconds>(duration).count();
    double seconds = static_cast<double>(millis / 1000.0);
    return Object(seconds);
  }
  size_t arity() const override { return 0; }
};

Interpreter::Interpreter() {
  // globals.define(const std::string &name, const LiteralType &value)
}

void Interpreter::interpret(const std::vector<Stmt *> statements) {
  try {
    for (Stmt *statement : statements) {
      statement->execute(this);
    }
  } catch (const RuntimeError &e) {
    handle_runtime_error(e);
  }
}
