#include <iostream>
#include <stdexcept>
#include <vector>

int main() {
    std::vector<int> prices = {10, 20, 30};
    try {
        std::cout << prices.at(9) << "\n";
    } catch (const std::out_of_range &e) {
        std::cout << "at(9): " << e.what() << "\n";
    }
}
