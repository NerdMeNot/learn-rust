#include <cassert>
#include <memory>
#include <type_traits>
#include <variant>
#include <vector>

struct Shape {
    virtual ~Shape() = default;
    virtual unsigned area() const = 0;
};

struct Square : Shape {
    unsigned side = 3;
    unsigned area() const override { return side * side; }
};

struct Rect : Shape {
    unsigned area() const override { return 24; }
};

struct Circle { unsigned r; };
struct Tri { unsigned b; };

int main() {
    std::vector<std::unique_ptr<Shape>> shapes;
    shapes.push_back(std::make_unique<Square>());
    shapes.push_back(std::make_unique<Rect>());
    const Shape& ref = *shapes[0];

    // dynamic_cast gives a pointer or nullptr
    assert(dynamic_cast<const Square*>(&ref) != nullptr);
    assert(dynamic_cast<const Square*>(shapes[1].get()) == nullptr);

    // std::variant with std::visit
    std::variant<Circle, Tri> v = Tri{5};
    unsigned got = std::visit([](auto s) -> unsigned {
        if constexpr (std::is_same_v<decltype(s), Circle>) return s.r;
        else return s.b;
    }, v);
    assert(got == 5);
    return 0;
}
