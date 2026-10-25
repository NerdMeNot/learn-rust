#include <cstdlib>
#include <functional>
#include <iostream>
#include <string>
#include <tuple>

constexpr int square(int n) { return n * n; }

int area(int w, int h = 1) { return w * h; }

int add(int a, int b) { return a + b; }
double add(double a, double b) { return a + b; }

int apply(std::function<int(int)> f, int x) { return f(x); }

[[noreturn]] void die() { std::abort(); }

std::size_t by_value(std::string s) {
    s += "!";
    return s.size();
}

std::size_t by_ref(const std::string& s) { return s.size(); }

void by_mut_ref(std::string& s) { s += "!"; }

int show_size(auto x) { return static_cast<int>(sizeof(x)); }

int main() {
    static_assert(square(12) == 144);
    int grid[square(3)] = {};
    if (sizeof(grid) / sizeof(grid[0]) != 9) std::abort();
    if (area(3) != 3 || area(3, 4) != 12) std::abort();
    if (add(1, 2) != 3 || add(1.5, 2.5) != 4.0) std::abort();
    int extra = 100;
    if (apply([extra](int n) { return n + extra; }, 4) != 104) std::abort();
    std::string name = "cpp";
    if (by_value(name) != 4 || name != "cpp") std::abort();  // by value copies
    if (by_ref(name) != 3) std::abort();
    by_mut_ref(name);
    if (name != "cpp!") std::abort();
    std::string moved = std::move(name);  // `name` is still a usable string afterwards
    if (moved != "cpp!" || name.size() > 4) std::abort();
    if (show_size(1) != 4) std::abort();
    if (false) die();
    std::cout << "all checks passed\n";
}
