#include <expected>
#include <iostream>
#include <string>

std::expected<int, std::string> parse_port(const std::string &s) {
    try {
        return std::stoi(s);
    } catch (...) {
        return std::unexpected("port must be a number, got \"" + s + "\"");
    }
}

int main() {
    auto r = parse_port("eighty");
    if (!r) std::cout << r.error() << "\n";
    else std::cout << *r << "\n";
}
