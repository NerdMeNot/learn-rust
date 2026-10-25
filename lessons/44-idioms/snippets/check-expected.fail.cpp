#include <expected>
#include <iostream>

std::expected<int, const char*> parse(int n) {
    if (n > 0) return n;
    return std::unexpected("not positive");
}

int main() {
    std::cout << parse(3).value() << "\n";
}
