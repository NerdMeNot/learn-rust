#include <iostream>
#include <numeric>
#include <ranges>
#include <vector>

int main() {
    std::vector<int> nums{1, 2, 3, 4, 5, 6};

    auto view = nums
        | std::views::filter([](int n) { return n % 2 == 0; })
        | std::views::transform([](int n) { return n * n; });   // lazy, like a Rust iterator
    std::vector<int> evens_squared(view.begin(), view.end());   // collect

    int total = std::accumulate(nums.begin(), nums.end(), 0);
    std::cout << "[";
    for (std::size_t i = 0; i < evens_squared.size(); ++i) {
        std::cout << (i ? ", " : "") << evens_squared[i];
    }
    std::cout << "] " << total << "\n";
}
