#include <iostream>
#include <variant>

struct Circle { double r; };
struct Rect { double w, h; };
using Shape = std::variant<Circle, Rect>;

int main() {
    Shape s = Circle{1.0};
    std::cout << "sizeof(Shape): " << sizeof(Shape) << "\n";
    std::cout << "index: " << s.index() << "\n";
    try {
        std::cout << std::get<Rect>(s).w << "\n";
    } catch (const std::bad_variant_access& e) {
        std::cout << "get<Rect>: " << e.what() << "\n";
    }
    if (auto* c = std::get_if<Circle>(&s)) {
        std::cout << "radius: " << c->r << "\n";
    }
}
