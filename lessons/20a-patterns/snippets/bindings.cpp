#include <iostream>
#include <map>
#include <string>
#include <tuple>

struct Point {
    int x, y;
};

int main() {
    Point p{3, -2};
    auto [x, y] = p;  // a copy, bound by position
    std::cout << "x = " << x << ", y = " << y << "\n";

    const auto& [cx, cy] = p;  // read-only names for the members of p itself
    std::cout << "cx = " << cx << ", cy = " << cy << "\n";

    auto& [rx, ry] = p;  // names for the members of p itself
    rx += 10;
    std::cout << "p.x after rx += 10: " << p.x << "\n";

    std::map<std::string, int> ages{{"ana", 30}, {"bo", 25}};
    for (const auto& [name, age] : ages) {
        std::cout << name << " is " << age << "\n";
    }

    auto [a, b] = std::tuple<int, std::string>{1, "one"};
    std::cout << a << " " << b << "\n";
}
