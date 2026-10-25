#include <iostream>

template <typename T>
T largest(T a, T b) {
    return a > b ? a : b;
}

struct Point { int x, y; };

int main() {
    std::cout << largest(3, 5) << "\n";
    largest(Point{1, 2}, Point{3, 4});
    return 0;
}
