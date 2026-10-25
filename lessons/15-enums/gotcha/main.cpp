#include <iostream>
#include <variant>

struct Circle { double r; };
struct Rect { double w, h; };
struct Triangle { double b, h; };   // new!
using Shape = std::variant<Circle, Rect, Triangle>;

template <class... Fs> struct overloaded : Fs... { using Fs::operator()...; };

double area(const Shape& s) {
    return std::visit(overloaded{
        [](const Circle& c) { return 3.14 * c.r * c.r; },
        [](const Rect& r) { return r.w * r.h; },
    }, s);
}

int main() {
    std::cout << area(Triangle{2.0, 3.0}) << "\n";
}
