#include <concepts>
#include <iostream>
#include <string>

template <typename T>
concept Speaker = requires(const T& t) {
    { t.speak() } -> std::convertible_to<std::string>;
};

struct Rock {};

template <typename T>
    requires Speaker<T>
void introduce(const T& s) {
    std::cout << s.speak() << "\n";
}

int main() {
    introduce(Rock{});
}
