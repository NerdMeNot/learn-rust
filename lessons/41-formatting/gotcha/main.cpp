#include <format>
#include <iostream>

struct Point {
    int x;
    int y;
};

int main() {
    Point p{1, 2};
    std::cout << std::format("{}\n", p);
}
