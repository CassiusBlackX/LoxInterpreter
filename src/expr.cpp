#include "expr.h"
#include "error.h"
#include <cassert>
#include <initializer_list>
#include <string>
#include <string_view>

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

static void check_number_operand(const Token &op, const LiteralType &operand) {
  if (operand.get_type() == LiteralType::Type::Number)
    return;
  throw RuntimeError(op, "Operand must be a Number");
}

static void check_number_operands(const Token &op, const LiteralType &operand1,
                                  const LiteralType &operand2) {
  if (operand1.get_type() == LiteralType::Type::Number &&
      operand2.get_type() == LiteralType::Type::Number)
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
  std::string identifier_name = name.get_lexeme();
  return parenthesize(identifier_name + " assignment", {value});
}

LiteralType Literal::evaluate(Environment *environment) { return value; }

LiteralType Variable::evaluate(Environment *environment) {
  return environment->get(name);
}

LiteralType Grouping::evaluate(Environment *environment) {
  return expr->evaluate(environment);
}

LiteralType Unary::evaluate(Environment *environment) {
  LiteralType right_value = right->evaluate(environment);
  switch (op.get_tokentype()) {
  case TokenType::Minus: {
    check_number_operand(op, right_value);
    double value = -static_cast<double>(right_value);
    return LiteralType(value);
  }
  case TokenType::Bang: {
    bool value = !static_cast<bool>(right_value);
    return LiteralType(value);
  }
  default:
    return LiteralType(); // return Nil by default
  }
}

LiteralType Binary::evaluate(Environment *environment) {
  LiteralType left_value = left->evaluate(environment);
  LiteralType right_value = right->evaluate(environment);

  switch (op.get_tokentype()) {
  case TokenType::Minus:
    check_number_operands(op, left_value, right_value);
    return LiteralType(static_cast<double>(left_value) -
                       static_cast<double>(right_value));
  case TokenType::Slash:
    check_number_operands(op, left_value, right_value);
    return LiteralType(static_cast<double>(left_value) /
                       static_cast<double>(right_value));
  case TokenType::Star:
    check_number_operands(op, left_value, right_value);
    return LiteralType(static_cast<double>(left_value) *
                       static_cast<double>(right_value));
  case TokenType::Plus:
    if (left_value.get_type() == LiteralType::Type::Number &&
        right_value.get_type() == LiteralType::Type::Number) {
      return LiteralType(static_cast<double>(left_value) +
                         static_cast<double>(right_value));
    }
    if (left_value.get_type() == LiteralType::Type::String &&
        right_value.get_type() == LiteralType::Type::String) {
      // Type::String can use `+` to append
      return LiteralType(left_value.to_string() + right_value.to_string(),
                         LiteralType::Type::String);
    }
    throw RuntimeError(op, "Operands must be two Number or two String");
  case TokenType::Greater:
    check_number_operands(op, left_value, right_value);
    return LiteralType(static_cast<double>(left_value) >
                       static_cast<double>(right_value));
  case TokenType::GreaterEqual:
    check_number_operands(op, left_value, right_value);
    return LiteralType(static_cast<double>(left_value) >=
                       static_cast<double>(right_value));
  case TokenType::Less:
    check_number_operands(op, left_value, right_value);
    return LiteralType(static_cast<double>(left_value) <
                       static_cast<double>(right_value));
  case TokenType::LessEqual:
    check_number_operands(op, left_value, right_value);
    return LiteralType(static_cast<double>(left_value) <=
                       static_cast<double>(right_value));
  case TokenType::EqualEqual:
    // reused the `==` operator
    return LiteralType(left_value == right_value);
  case TokenType::BangEqual:
    // reused the `==` operator
    return LiteralType(!(left_value == right_value));
  default:
    return LiteralType(); // return Nil by default
  }
}

LiteralType Assign::evaluate(Environment *environment) {
  LiteralType value_ = value->evaluate(environment);
  environment->assign(name, value_);
  return value;
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
  } else if (auto assign = dynamic_cast<Assign*>(expr)) {
    delete_expr(assign->value);
    assign->value = nullptr;
  } else {
    // Literal has no child node, we can ignore it
    // Variable does not allocate on heap, can ignore it as well 
  }

  delete expr;
  expr = nullptr;
}
