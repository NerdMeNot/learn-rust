#include <format>
#include <iostream>
#include <locale>

struct German : std::numpunct<char> {
    char do_decimal_point() const override { return ','; }
    char do_thousands_sep() const override { return '.'; }
    std::string do_grouping() const override { return "\3"; }
};

int main() {
    std::locale de(std::locale::classic(), new German);
    std::cout << std::format("{}", 1234567.5) << "\n";         // never uses a locale
    std::cout << std::format("{:L}", 1234567.5) << "\n";        // the global locale (classic)
    std::cout << std::format(de, "{:L}", 1234567.5) << "\n";    // the locale we built
    std::cout << std::format(de, "{}", 1234567.5) << "\n";
}
