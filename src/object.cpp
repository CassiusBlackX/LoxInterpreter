#include <iomanip>
#include <sstream>

#include "object.h"

#ifndef FLOAT_PRECISION
#define FLOAT_PRECISION 4
#endif

std::string Object::to_string() const {
  return std::visit(
      [](auto &&arg) -> std::string {
        using T = std::decay_t<decltype(arg)>;
        if constexpr (std::is_same_v<T, double>) {
          std::ostringstream oss;
          oss << std::fixed << std::setprecision(FLOAT_PRECISION);
          oss << arg;
          return oss.str();
        } else if constexpr (std::is_same_v<T, bool>)
          return arg ? "true" : "false";
        else if constexpr (std::is_same_v<T, std::nullptr_t>)
          return "nil";
        else if constexpr (std::is_same_v<T, std::string>)
          return std::string(arg);
        else
          return "";
      },
      value_);
}

bool Object::operator==(const Object &other) const {
  if (type_ != other.type_)
    return false;

  switch (type_) {
  case Type::Nil:
    return true;
  case Type::Number:
    return std::get<double>(value_) == std::get<double>(other.value_);
  case Type::Bool:
    return std::get<bool>(value_) == std::get<bool>(other.value_);
  case Type::String:
  case Type::Identifer:
    return std::get<std::string>(value_) == std::get<std::string>(other.value_);
  default:
    return false;
  }
}

Object::operator double() const {
  if (type_ != Type::Number) {
    throw std::runtime_error(
        "Cannot convert non-Number LiteralType into double");
  }
  return std::get<double>(value_);
}

// everything else but Nil and false is true
Object::operator bool() const {
  switch (type_) {
  case Type::String:
  case Type::Number:
    return true;
  case Type::Nil:
    return false;
  case Type::Identifer:
    throw std::runtime_error("should not convert an identifier into a bool");
  case Type::Bool:
    return std::get<bool>(value_);
  }
}

// BUG: should Type::Nil be able to accept other values?
Object::operator std::nullptr_t() const {
  if (type_ != Type::Nil) {
    throw std::runtime_error("Cannot convert non-Nil LiteralType into Nil");
  }
  return std::get<std::nullptr_t>(value_);
}
