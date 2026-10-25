#include <iostream>
#include <ranges>
#include <string_view>

int main() {
    std::string_view line = "ana,30,lisbon";
    auto parts = line | std::views::split(',');
    auto name = parts[0];
    std::cout << std::string_view(name.begin(), name.end()) << "\n";
}
