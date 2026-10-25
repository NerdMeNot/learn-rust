#include <cstddef>
#include <cstdint>
#include <iostream>

struct S {
    std::uint8_t a;
    std::uint32_t b;
    std::uint8_t c;
};

int main() {
    std::cout << "S: size " << sizeof(S) << ", align " << alignof(S) << "\n";
    std::cout << "  offsets: a " << offsetof(S, a) << ", b " << offsetof(S, b)
              << ", c " << offsetof(S, c) << "\n";
}
