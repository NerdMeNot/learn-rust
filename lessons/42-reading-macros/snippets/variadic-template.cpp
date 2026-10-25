#include <iostream>

// A parameter pack takes any number of arguments, and the types may differ.
template <typename... Ts>
auto sum(Ts... xs) {
    return (xs + ...); // a fold expression (C++17)
}

template <typename T>
constexpr T square(T x) {
    return x * x;
}

static_assert(square(7) == 49); // checked while compiling

int main() {
    std::cout << "sum(1, 2, 3, 4) = " << sum(1, 2, 3, 4) << "\n";
    std::cout << "sum(1.5, 2, 3L) = " << sum(1.5, 2, 3L) << "\n";
    std::cout << "square(2.5) = " << square(2.5) << "\n";
    return 0;
}
