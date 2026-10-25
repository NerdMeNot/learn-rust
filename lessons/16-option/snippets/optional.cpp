#include <iostream>
#include <optional>

int main() {
    std::optional<int> none;

    std::cout << "none.has_value(): " << none.has_value() << "\n";
    std::cout << "none.value_or(-1): " << none.value_or(-1) << "\n";

    // value() checks. (*none and none-> do not, and are undefined on an empty optional.)
    try {
        std::cout << none.value() << "\n";
    } catch (const std::bad_optional_access& e) {
        std::cout << "none.value() threw: " << e.what() << "\n";
    }

    // A pointer that may be null is not smaller than a plain pointer. An optional of one is.
    std::cout << "sizeof(int*): " << sizeof(int *) << "\n";
    std::cout << "sizeof(std::optional<int*>): " << sizeof(std::optional<int *>) << "\n";
    std::cout << "sizeof(int): " << sizeof(int) << "\n";
    std::cout << "sizeof(std::optional<int>): " << sizeof(std::optional<int>) << "\n";
}
