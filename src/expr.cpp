#include "expr.h"
#include <initializer_list>
#include <string>
#include <string_view>

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
  } else {
    // Literal has no child node, we can ignore it
  }

  delete expr;
  expr = nullptr;
}

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

std::string Literal::print() { return value.to_string(); }

std::string Grouping::print() { return parenthesize("group", {expr}); }

std::string Unary::print() { return parenthesize(op.get_lexeme(), {right}); }

std::string Binary::print() {
  return parenthesize(op.get_lexeme(), {left, right});
}
