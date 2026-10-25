#include <algorithm>
#include <cassert>
#include <string>
#include <string_view>
#include <tuple>
#include <utility>
#include <vector>

constexpr int MAX = 100;

int add(int a, int b) { return a + b; }

int main() {
    int x = 5;
    const int y = 5;
    auto z = 5;
    const auto w = 5;
    assert(x == y && z == w && MAX == 100 && add(1, 2) == 3);

    auto pair = std::make_pair(1, 2);
    auto [p, q] = pair;
    assert(p == 1 && q == 2);

    double price = 2.9;
    assert(static_cast<int>(price) == 2 && static_cast<int>(-price) == -2 && (int)price == 2);

    auto inc = [](int v) { return v + 1; };
    assert(inc(1) == 2);

    std::vector<int> v;
    std::vector<int> v2 = std::vector<int>();
    assert(v.empty() && v2.empty());

    const int n = 3;
    int grid[n]; // legal in C++: a const int with a constant initialiser is a constant expression
    grid[0] = 1;
    double d = 1; // implicit conversion
    int truncated = 5.7;
    assert(grid[0] == 1 && d == 1.0 && truncated == 5);
    std::string s = "Ada";
    std::string_view sv = "Ada";
    assert(s == sv && std::stoi("42") == 42 && std::max<int>(1, 2) == 2);
    auto small = 7;
    unsigned long total = small + 1; // `auto` fixed `small` as int from its initialiser
    assert(total == 8 && sizeof(small) == sizeof(int));

    static_assert(sizeof('a') == 1, "in C++ a character literal is a char");
    assert(0377 == 255 && 017 == 15);
    assert(1'000'000 == 1000000 && 0xff == 255 && 0b101 == 5);
    assert(5u == 5 && 5UL == 5 && 2.5f == 2.5);
    return 0;
}
