#include <iostream>
#include <tuple>
#include <utility>
#include <vector>

std::pair<int, int> min_max(const std::vector<int>& xs) {
    int lo = xs[0];
    int hi = xs[0];
    for (int x : xs) {
        if (x < lo) lo = x;
        if (x > hi) hi = x;
    }
    return {lo, hi};
}

int main() {
    auto [lo, hi] = min_max({4, 9, 1, 7});   // structured binding
    std::cout << lo << " " << hi << "\n";

    int only_hi;
    std::tie(std::ignore, only_hi) = min_max({3, 5});   // std::ignore skips a value
    std::cout << only_hi << "\n";
}
