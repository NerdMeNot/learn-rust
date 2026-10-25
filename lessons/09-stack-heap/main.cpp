#include <iostream>

struct Point {
    int x, y;
};

int main() {
    Point a{1, 2};
    Point b = a; // a struct is a value: b is a copy
    b.x = 99;
    std::cout << "a = (" << a.x << ", " << a.y << ")\n";
    std::cout << "b = (" << b.x << ", " << b.y << ")\n";

    // To share, you take a reference on purpose.
    Point &r = a;
    r.x = 50;
    std::cout << "a after r.x = 50: (" << a.x << ", " << a.y << ")\n";
}
