#include <format>
#include <iostream>
#include <string>

struct Money {
    long cents;
};

// a std::formatter specialization: how Money looks with {}
template <>
struct std::formatter<Money> : std::formatter<std::string> {
    auto format(const Money &m, std::format_context &ctx) const {
        return std::format_to(ctx.out(), "${}.{:02}", m.cents / 100, m.cents % 100);
    }
};

struct Item {
    std::string name;
    unsigned qty;
};

// no derive: operator<< is written by hand
std::ostream &operator<<(std::ostream &os, const Item &item) {
    return os << "Item { name: \"" << item.name << "\", qty: " << item.qty << " }";
}

int main() {
    int n = 42;
    std::cout << std::format("[{:>6}] [{:<6}] [{:^6}] [{:06}]\n", n, n, n, n);
    std::cout << std::format("{:.2f} {:e} {:#x} {:#b}\n", 3.14159, 1234.5, n, n);

    Money price{1999};
    std::cout << std::format("total: {}\n", price);

    std::string label = std::format("{} each", price); // std::format builds a std::string
    std::cout << std::format("{} ({} chars)\n", label, label.size());

    Item item{"pen", 3};
    std::cout << std::format("{} x {}\n", item.qty, item.name);
    std::cout << item << "\n";
}
