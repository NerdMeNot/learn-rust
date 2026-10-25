#include <cstdint>
#include <iostream>

std::uint8_t takes_u8(std::uint8_t n) { return n; }

int main() {
    auto a = 5; // auto looks only at the right-hand side: int
    takes_u8(a);
    std::cout << "a: " << sizeof(a) << " byte(s)\n";
}
