#include <cstdint>
#include <iostream>
#include <utility>

int main() {
    std::uint8_t a = 200, b = 100;
    auto sum = a + b;
    std::cout << "sizeof(a + b): " << sizeof(sum) << "\n";
    std::cout << "a + b: " << sum << "\n";
    std::uint8_t c = a + b;
    std::cout << "stored in a uint8_t: " << static_cast<int>(c) << "\n";

    int neg = -1;
    unsigned one = 1;
    std::cout << "-1 < 1u: " << (neg < one) << "\n";

    std::cout << "std::cmp_less(-1, 1u): " << std::cmp_less(neg, one) << "\n";

    char ch = static_cast<char>(200);
    std::cout << "(char)200: " << static_cast<int>(ch) << "\n";
}
