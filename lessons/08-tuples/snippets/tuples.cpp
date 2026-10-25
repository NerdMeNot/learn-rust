#include <iostream>
#include <tuple>

std::tuple<int, int> divmod(int a, int b) { return {a / b, a % b}; }

int main() {
    auto t = divmod(17, 5);
    std::cout << "get<0>: " << std::get<0>(t) << ", get<1>: " << std::get<1>(t) << "\n";

    auto [q, r] = t;
    std::cout << "binding: " << q << " " << r << "\n";

    int a = 1, b = 2;
    std::tie(a, b) = std::make_tuple(b, a);
    std::cout << "tie swap: " << a << " " << b << "\n";

    std::cout << "tuple(1, 9) < tuple(2, 0): " << (std::make_tuple(1, 9) < std::make_tuple(2, 0)) << "\n";
}
