#include <cstdint>
#include <iostream>
#include <string>

constexpr std::uint32_t LIMIT = 1'000;

double half(double x) {
    return x / 2.0;
}

int main() {
    std::string name = "Ada";
    std::uint8_t visits = 0;
    visits += 1;
    int mask = 0xff;
    float ratio = 2.5f;
    char letter = 'R';
    double total = static_cast<double>(LIMIT) + half(5.0);
    int parsed = std::stoi("42");

    std::cout << "name=" << name << " visits=" << int(visits) << "\n";
    std::cout << "mask=" << mask << " ratio=" << ratio << " letter=" << letter << "\n";
    std::cout << "limit=" << LIMIT << " total=" << total << "\n";
    std::cout << "parsed=" << parsed << "\n";
}
