#include <array>
#include <cstddef>
#include <cassert>
#include <concepts>
#include <iostream>
#include <string>

// Backs the C++ cells of the table in lesson.mdx. Never shown.
template <typename T, std::size_t N>
T sum(const std::array<T, N>& values) {
    T total{};
    for (const T& v : values) total += v;
    return total;
}

template <typename T>
T biggest(T a, T b) { return a > b ? a : b; }

template <typename T>
struct Describe { static const char* name() { return "any"; } };

template <>
struct Describe<int> { static const char* name() { return "int"; } };

// C++20 abbreviated function template: a hidden type parameter.
void show(std::totally_ordered auto x) { std::cout << x << "\n"; }

int main() {
    show(1);
    assert((sum(std::array<int, 3>{1, 2, 3}) == 6));
    assert(biggest<double>(1, 2.5) == 2.5);
    assert(std::string(Describe<int>::name()) == "int");
    assert(std::string(Describe<char>::name()) == "any");
    std::cout << "ok\n";
}
