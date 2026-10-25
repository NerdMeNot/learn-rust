#include <charconv>
#include <iostream>
#include <ranges>
#include <string_view>

int main() {
    using namespace std::literals;
    for (std::string_view text : {"a,b,"sv, ""sv, "a,,b"sv}) {
        int pieces = 0;
        std::cout << "\"" << text << "\":";
        for (auto part : text | std::views::split(',')) {
            std::cout << " [" << std::string_view(part.begin(), part.end()) << "]";
            pieces++;
        }
        std::cout << "  (" << pieces << " pieces)\n";
    }

    for (std::string_view text : {"42"sv, "+42"sv, " 42"sv, "42abc"sv}) {
        int n = 0;
        auto [end, ec] = std::from_chars(text.data(), text.data() + text.size(), n);
        std::cout << "from_chars(\"" << text << "\"): " << (ec == std::errc() ? "ok " : "error ") << n
                  << ", consumed " << (end - text.data()) << " of " << text.size() << "\n";
    }
}
