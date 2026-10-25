#include <functional>
#include <iostream>
#include <string>
#include <utility>

int main() {
    int x = 1;
    auto by_value = [x] { return x; };
    auto by_ref = [&x] { return x; };
    auto by_value_mutable = [x]() mutable { return ++x; };
    x = 2;
    std::cout << "[x]: " << by_value() << ", [&x]: " << by_ref() << "\n";
    std::cout << "[x] mutable: " << by_value_mutable() << " then " << by_value_mutable() << ", x is " << x << "\n";

    std::string s = "ferris";
    auto owns = [s = std::move(s)] { return s.size(); };
    std::cout << "moved in: " << owns() << ", s.size() now " << s.size() << "\n";

    std::function<int()> counter = [n = 0]() mutable { return ++n; };
    std::cout << "std::function holding a mutable lambda: " << counter() << " then " << counter() << "\n";

    int y = 10;
    auto mixed = [x, &y] { y += x; return y; };
    std::cout << "[x, &y]: " << mixed() << ", y is " << y << "\n";
}
