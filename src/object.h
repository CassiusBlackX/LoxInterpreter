#ifndef OBJECT_H_
#define OBJECT_H_

#include <cassert>
#include <string>
#include <variant>

class Callable;

class Object {
public:
  enum class Type {
    Identifer,
    String,
    Bool,
    Nil,
    Number,
    CallablePtr,
  };

  Object() : value_(nullptr), type_(Type::Nil) {}
  Object(double d) : value_(d), type_(Type::Number) {}
  Object(bool b) : value_(b), type_(Type::Bool) {}
  Object(std::string str, Type ty) : value_(std::move(str)), type_(ty) {
    assert(ty == Type::String || ty == Type::Identifer);
  }
  Object(Callable *ptr) : value_(ptr), type_(Type::CallablePtr) {}

  bool operator==(const Object &other) const;
  std::string to_string() const;
  Type get_type() const { return type_; }

  explicit operator double() const;
  explicit operator bool() const;
  explicit operator std::nullptr_t() const;

  friend std::ostream &operator<<(std::ostream &os, Object literal) {
    os << literal.to_string();
    return os;
  }

private:
  std::variant<double, bool, std::string, std::nullptr_t, Callable *> value_;
  Type type_;
};

#endif // OBJECT_H_
