#include <chrono>
#include <expected>
#include <iostream>
#include <regex>

std::expected<int, std::string> parse(const std::string &s) {
    try { return std::stoi(s); } catch (...) { return std::unexpected("not a number"); }
}

int main() {
    std::smatch m;
    std::string text = "shipped on 2026-10-25";
    bool found = std::regex_search(text, m, std::regex("[0-9]{4}-[0-9]{2}-[0-9]{2}"));
    std::chrono::year_month_day d{std::chrono::year{2026}, std::chrono::month{10}, std::chrono::day{25}};
    std::chrono::weekday w{std::chrono::sys_days{d}};
    std::cout << "C++: regex " << (found ? m.str() : "no match") << ", chrono weekday " << w.c_encoding() << " (0 = Sunday), expected " << parse("eighty").error() << "\n";
}
