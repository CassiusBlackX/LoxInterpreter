#include "expr.h"
#include "error.h"
#include "interpreter.h"
#include "object.h"
#include <cassert>
#include <initializer_list>
#include <sstream>
#include <string>
#include <string_view>
#include <vector>

static std::string parenthesize(const std::string_view name,
                                std::initializer_list<Expr *> exprs) {
  std::string result = "(";
  result += name;
  for (const auto expr : exprs) {
    result += ' ';
    result += expr->print();
  }
  return result + ')';
}

static void check_number_operand(const Token &op, const Object &operand) {
  if (operand.get_type() == Object::Type::Number)
    return;
  throw RuntimeError(op, "Operand must be a Number");
}

static void check_number_operands(const Token &op, const Object &operand1,
                                  const Object &operand2) {
  if (operand1.get_type() == Object::Type::Number &&
      operand2.get_type() == Object::Type::Number)
    return;
  throw RuntimeError(op, "Operands must be two Number");
}

std::string Literal::print() const { return value.to_string(); }

std::string Grouping::print() const { return parenthesize("group", {expr}); }

std::string Unary::print() const {
  return parenthesize(op.get_lexeme(), {right});
}

std::string Binary::print() const {
  return parenthesize(op.get_lexeme(), {left, right});
}

std::string Variable::print() const { return name.to_string(); }

std::string Assign::print() const {
  std::string identifier_name = target->name.get_lexeme();
  return parenthesize(identifier_name + " assignment", {value});
}

std::string Logical::print() const {
  return parenthesize(op.get_lexeme(), {left, right});
}

std::string Call::print() const {
  std::string func_name = callee->print();
  return "Call " + func_name;
}

Object Literal::evaluate(Interpreter *interpreter) { return value; }

Object Variable::evaluate(Interpreter *interpreter) {
  return interpreter->get_current()->get(name);
}

Object Grouping::evaluate(Interpreter *interpreter) {
  // return expr->evaluate(environment);
  return expr->evaluate(interpreter);
}

Object Unary::evaluate(Interpreter *interpreter) {
  Object right_value = right->evaluate(interpreter);
  switch (op.get_tokentype()) {
  case TokenType::Minus: {
    check_number_operand(op, right_value);
    double value = -static_cast<double>(right_value);
    return Object(value);
  }
  case TokenType::Bang: {
    bool value = !static_cast<bool>(right_value);
    return Object(value);
  }
  default:
    return Object(); // return Nil by default
  }
}

Object Binary::evaluate(Interpreter *interpreter) {
  Object left_value = left->evaluate(interpreter);
  Object right_value = right->evaluate(interpreter);

  switch (op.get_tokentype()) {
  case TokenType::Minus:
    check_number_operands(op, left_value, right_value);
    return Object(static_cast<double>(left_value) -
                  static_cast<double>(right_value));
  case TokenType::Slash:
    check_number_operands(op, left_value, right_value);
    return Object(static_cast<double>(left_value) /
                  static_cast<double>(right_value));
  case TokenType::Star:
    check_number_operands(op, left_value, right_value);
    return Object(static_cast<double>(left_value) *
                  static_cast<double>(right_value));
  case TokenType::Plus:
    if (left_value.get_type() == Object::Type::Number &&
        right_value.get_type() == Object::Type::Number) {
      return Object(static_cast<double>(left_value) +
                    static_cast<double>(right_value));
    }
    if (left_value.get_type() == Object::Type::String &&
        right_value.get_type() == Object::Type::String) {
      // Type::String can use `+` to append
      return Object(left_value.to_string() + right_value.to_string(),
                    Object::Type::String);
    }
    throw RuntimeError(op, "Operands must be two Number or two String");
  case TokenType::Greater:
    check_number_operands(op, left_value, right_value);
    return Object(static_cast<double>(left_value) >
                  static_cast<double>(right_value));
  case TokenType::GreaterEqual:
    check_number_operands(op, left_value, right_value);
    return Object(static_cast<double>(left_value) >=
                  static_cast<double>(right_value));
  case TokenType::Less:
    check_number_operands(op, left_value, right_value);
    return Object(static_cast<double>(left_value) <
                  static_cast<double>(right_value));
  case TokenType::LessEqual:
    check_number_operands(op, left_value, right_value);
    return Object(static_cast<double>(left_value) <=
                  static_cast<double>(right_value));
  case TokenType::EqualEqual:
    // reused the `==` operator
    return Object(left_value == right_value);
  case TokenType::BangEqual:
    // reused the `==` operator
    return Object(!(left_value == right_value));
  default:
    return Object(); // return Nil by default
  }
}

Object Assign::evaluate(Interpreter *interpreter) {
  Object value_ = value->evaluate(interpreter);
  interpreter->get_current()->assign(target->name, value_);
  return value;
}

Object Logical::evaluate(Interpreter *interpreter) {
  Object left_vale = left->evaluate(interpreter);
  // 'and', 'or' are short-circuit
  if (op.get_tokentype() == TokenType::Or) {
    if (static_cast<bool>(left_vale))
      return static_cast<bool>(left_vale);
  } else {
    if (!static_cast<bool>(left_vale))
      return static_cast<bool>(left_vale);
  }

  return static_cast<bool>(right->evaluate(interpreter));
}

Object Call::evaluate(Interpreter *interpreter) {
  return Object();
  // Object callee_value = callee->evaluate(interpreter);
  // std::vector<Object> arguments_value;
  // arguments_value.reserve(arguments.size());
  // for (Expr *arg : arguments) {
  //   arguments_value.push_back(arg->evaluate(interpreter));
  // }
  //
  // if (auto callable = dynamic_cast<Callable *>(callee)) {
  //   if (arguments_value.size() == callable->arity()) {
  //     return callable->call(environment, arguments_value);
  //   } else {
  //     std::ostringstream oss;
  //     oss << "Expected " << callable->arity() << " arguments, but got "
  //         << arguments_value.size() << ".";
  //     throw RuntimeError(paren, oss.str());
  //   }
  // } else {
  //   throw RuntimeError(paren, "can only call functions and classes.");
  // }
}

void delete_expr(Expr *expr) {
  if (expr == nullptr)
    return;

  if (auto binary = dynamic_cast<Binary *>(expr)) {
    delete_expr(binary->left);
    binary->left = nullptr;
    delete_expr(binary->right);
    binary->right = nullptr;
  } else if (auto unary = dynamic_cast<Unary *>(expr)) {
    delete_expr(unary->right);
    unary->right = nullptr;
  } else if (auto grouping = dynamic_cast<Grouping *>(expr)) {
    delete_expr(grouping->expr);
    grouping->expr = nullptr;
  } else if (auto assign = dynamic_cast<Assign *>(expr)) {
    delete_expr(assign->target);
    assign->target = nullptr;
    delete_expr(assign->value);
    assign->value = nullptr;
  } else if (auto logical_expr = dynamic_cast<Logical *>(expr)) {
    delete_expr(logical_expr->left);
    logical_expr->left = nullptr;
    delete_expr(logical_expr->right);
    logical_expr->right = nullptr;
  } else {
    // Literal has no child node, do nothing here
    // Variable has no child node, do nothing here
  }

  delete expr;
  expr = nullptr;
}
