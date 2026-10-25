#include <algorithm>
#include <cctype>
#include <charconv>
#include <format>
#include <iostream>
#include <ranges>
#include <string>
#include <string_view>
#include <vector>

// std has no trim: move the ends of a view inward
std::string_view trim(std::string_view s) {
    auto start = s.find_first_not_of(' ');
    if (start == std::string_view::npos) return {};
    return s.substr(start, s.find_last_not_of(' ') - start + 1);
}

int main() {
    std::string_view line = "  name=Ana, age=30, city=Lisbon  ";

    auto clean = trim(line); // a view into line: nothing copied
    std::cout << "[" << clean << "]\n";

    // split on ',', trim each piece, split once on '='
    for (auto part : clean | std::views::split(',')) {
        auto field = trim(std::string_view(part.begin(), part.end()));
        if (auto eq = field.find('='); eq != std::string_view::npos) {
            std::cout << std::format("{:>5} -> {}\n", field.substr(0, eq), field.substr(eq + 1));
        }
    }

    // text to number: from_chars reports errors without exceptions
    std::string_view digits = "30";
    unsigned age = 0;
    if (std::from_chars(digits.data(), digits.data() + digits.size(), age).ec != std::errc{}) age = 0;
    std::cout << "next year: " << age + 1 << "\n";

    // these build new strings
    std::string shout(clean);
    std::ranges::transform(shout, shout.begin(), [](unsigned char c) { return std::toupper(c); });
    for (size_t at; (at = shout.find(", ")) != std::string::npos;) shout.replace(at, 2, " | ");
    std::cout << shout << "\n";

    std::cout << std::boolalpha << "starts with name: " << clean.starts_with("name") << "\n";
    auto pos = clean.find("age"); // npos when not found
    std::cout << "position of age: " << (pos == std::string_view::npos ? -1 : static_cast<long>(pos)) << "\n";

    std::vector<std::string> words{"build", "with", "rust"};
    std::string joined;
    for (const auto &w : words) joined += (joined.empty() ? "" : "-") + w;
    std::cout << joined << "\n";

    // grow a string with +=
    std::string out;
    for (int i = 1; i <= 3; i++) {
        out += std::to_string(i);
        out += ';';
    }
    std::cout << out << "\n";
}
