#include <format>
#include <iomanip>
#include <iostream>

struct Id {
    int n;
};

std::ostream& operator<<(std::ostream& os, const Id& id) {
    return os << '#' << id.n;  // two separate insertions
}

struct Id2 {
    int n;
};

template <>
struct std::formatter<Id2> : std::formatter<std::string> {
    auto format(const Id2& id, std::format_context& ctx) const {
        return std::formatter<std::string>::format("#" + std::to_string(id.n), ctx);
    }
};

int main() {
    std::cout << '[' << std::setw(8) << Id{42} << "]\n";
    std::cout << std::format("[{:>8}]\n", Id2{42});
    std::cout << std::format("{:.0f} {:.0f} {:.0f}\n", 0.5, 1.5, 2.5);
}
