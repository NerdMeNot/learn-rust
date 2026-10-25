#include <cstdint>
#include <iostream>

int main() {
    std::int32_t a = 7;      // 32-bit signed integer
    std::int64_t b = 10;     // 64-bit signed integer
    double price = 2.5;      // 64-bit float
    bool ok = true;

    auto total = static_cast<std::int64_t>(a) + b;   // explicit conversion
    auto cost = static_cast<double>(a) * price;
    std::cout << total << " " << cost << " " << std::boolalpha << ok << "\n";
}
