#include <array>
#include <cstdio>
#include <string>

static std::string get(const std::array<int, 3> &a, std::size_t i) {
    return i < a.size() ? "Some(" + std::to_string(a[i]) + ")" : "None";
}

int main() {
    std::array<int, 3> prices = {10, 20, 30};
    const int *p = prices.data();
    int second = *(p + 1);
    std::printf("second, read by offset: %d\n", second);
    std::printf("get(1): %s\n", get(prices, 1).c_str());
    std::printf("get(9): %s\n", get(prices, 9).c_str());
}
