#include <charconv>
#include <iostream>
#include <stdexcept>
#include <string_view>

unsigned parse_age(std::string_view s) {
    unsigned n = 0;
    auto [end, ec] = std::from_chars(s.data(), s.data() + s.size(), n);
    if (ec != std::errc{} || end != s.data() + s.size()) {
        throw std::invalid_argument("invalid digit found in string");
    }
    return n;
}

int main() {
    for (std::string_view input : {"42", "abc"}) {
        try {
            unsigned n = parse_age(input);   // a failure throws and jumps to catch
            std::cout << "age " << n << "\n";
        } catch (const std::invalid_argument& e) {
            std::cout << "error: " << e.what() << "\n";
        }
    }
}
