#include <compare>
#include <concepts>
#include <iostream>

// The concept promises ordering. The body also uses `+`, which it never promised.
template <std::totally_ordered T>
T twice(T a) {
    return a + a;
}

// Orderable, but no `+`.
struct Point {
    int x;
    auto operator<=>(const Point&) const = default;
};

int main() {
    std::cout << "twice(2) = " << twice(2) << "\n";
    // twice(Point{1}) is never written, so the template is never instantiated with
    // a Point: the program compiles.
}
