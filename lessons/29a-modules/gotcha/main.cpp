#include <iostream>

namespace shop {
namespace {
int discount_percent(int qty) {
    return qty >= 10 ? 20 : 0;
}
}  // namespace

int total(int qty) {
    int full = qty * 10;
    return full - full * discount_percent(qty) / 100;
}
}  // namespace shop

int main() {
    std::cout << "total: " << shop::total(12) << "\n";
    std::cout << "discount: " << shop::discount_percent(12) << "%\n";
}
