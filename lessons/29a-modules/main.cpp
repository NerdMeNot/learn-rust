#include <iostream>

namespace shop {
namespace {
// Unnamed namespace: private to this file.
int discount_percent(int qty) {
    return qty >= 10 ? 20 : 0;
}
}  // namespace

constexpr int price = 10;

int total(int qty) {
    int full = qty * price;
    return full - full * discount_percent(qty) / 100;
}
}  // namespace shop

int main() {
    std::cout << "3 items: " << shop::total(3) << "\n";
    std::cout << "12 items: " << shop::total(12) << "\n";
}
