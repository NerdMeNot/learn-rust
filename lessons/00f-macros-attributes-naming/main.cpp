#include <format>
#include <iostream>
#include <string>
#include <vector>

// A point on a grid.
struct Point {
    int x;
    int y;
};

int main() {
    Point p{3, 4}; // a line comment
    std::vector<int> items{1, 2, 3};
    if (items.size() != 3) return 1;
    std::cout << std::format("point: Point {{ x: {}, y: {} }}\n", p.x, p.y);
    std::cout << std::format("items: [{}, {}, {}]\n", items[0], items[1], items[2]);
    std::cout << std::format("row: [{:>5}] [{:<5}] [{:05}]\n", 42, 42, 42);
    std::string pi = std::format("{:.2f}", 3.14159);
    std::cout << std::format("pi: {} {{done}}\n", pi);
    return 0;
}
