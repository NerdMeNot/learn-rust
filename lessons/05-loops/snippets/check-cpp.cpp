#include <cassert>
#include <cstddef>
#include <cstdint>
#include <iostream>
#include <ranges>
#include <vector>

int main() {
    // views::iota(1, 4) is the half-open range 1, 2, 3
    int sum = 0;
    for (int i : std::views::iota(1, 4)) {
        sum += i;
    }
    assert(sum == 6);

    std::vector<int> xs{1, 2, 3};
    int total = 0;
    for (auto x : xs) {
        total += x;
    }
    assert(total == 6);

    // auto x copies each element: changing x leaves xs alone
    for (auto x : xs) {
        x *= 2;
    }
    assert(xs[0] == 1);
    for (auto& x : xs) {
        x *= 2;
    }
    assert(xs[0] == 2);

    // unsigned arithmetic wraps, so size_t i >= 0 never fails
    std::size_t i = 0;
    i--;
    assert(i == SIZE_MAX);
    std::cout << "ok\n";
}
