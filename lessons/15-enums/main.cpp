#include <iostream>
#include <variant>
#include <vector>

struct Circle { double r; };
struct Rect { double w, h; };
using Shape = std::variant<Circle, Rect>;

// a small helper: one lambda per alternative
template <class... Fs> struct overloaded : Fs... { using Fs::operator()...; };

double area(const Shape& s) {
    return std::visit(overloaded{
        [](const Circle& c) { return 3.14 * c.r * c.r; },
        [](const Rect& r) { return r.w * r.h; },
    }, s);
}

int main() {
    std::vector<Shape> shapes{Circle{1.0}, Rect{2.0, 3.0}};
    for (const auto& s : shapes) {
        std::cout << area(s) << "\n";
    }
}
