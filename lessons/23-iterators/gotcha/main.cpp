#include <iostream>
#include <ranges>
#include <vector>

int main() {
    std::vector<int> nums{1, 2, 3, 4, 5, 6};
    auto evens = nums | std::views::filter([](auto n) { return n % 2 == 0; });
    for (auto n : evens) {
        std::cout << n << " ";
    }
    std::cout << "\n";
}
