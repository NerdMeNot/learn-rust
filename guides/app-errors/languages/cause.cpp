#include <exception>
#include <iostream>
#include <stdexcept>
#include <string>

void load() {
    try {
        std::stoi("eighty");
    } catch (...) {
        std::throw_with_nested(std::runtime_error("reading bad.conf"));
    }
}

int main() {
    try {
        load();
    } catch (const std::exception &e) {
        std::cout << e.what();
        try {
            std::rethrow_if_nested(e);
        } catch (const std::exception &cause) {
            std::cout << ": " << cause.what() << "\n";
        }
    }
}
