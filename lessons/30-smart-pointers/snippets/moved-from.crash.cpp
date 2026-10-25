#include <iostream>
#include <memory>

int main() {
    auto a = std::make_unique<int>(5);
    auto b = std::move(a);
    std::cout << "b: " << *b << "\n";
    std::cout << "a is null: " << (a == nullptr ? "true" : "false") << "\n";
    std::cout << "a: " << *a << "\n"; // compiles; undefined behaviour (it crashed here)
}
