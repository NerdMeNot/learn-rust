#include <array>
#include <cstdio>

int main() {
    std::array<int, 3> prices = {10, 20, 30};
    const int *p = prices.data();
    int second = *(p + 1);
    std::printf("second: %d\n", second);
}
