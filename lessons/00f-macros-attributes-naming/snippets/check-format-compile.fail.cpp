// Backs the claim that C++20 std::format checks its format string when compiling. Never shown.
#include <format>
#include <iostream>

int main() {
    std::cout << std::format("{} and {}", 1) << "\n";
    return 0;
}
