#include <concepts>
#include <iostream>
#include <string>

template <typename T>
concept Speaker = requires(const T& t) {
    { t.speak() } -> std::convertible_to<std::string>;
};

// No common base class: any type with a matching speak() qualifies.
struct Dog {
    std::string speak() const { return "woof"; }
};
struct Robot {
    std::string speak() const { return "beep"; }
};

template <typename T>
    requires Speaker<T>
void introduce(const T& s) {
    std::cout << s.speak() << "\n";
}

int main() {
    introduce(Dog{});
    introduce(Robot{});
}
