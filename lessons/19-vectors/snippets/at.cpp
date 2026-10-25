#include <iostream>
#include <stdexcept>
#include <vector>

int main() {
    std::vector<int> v{10, 20, 30};
    try {
        std::cout << v.at(10) << "\n";
    } catch (const std::out_of_range&) {
        std::cout << "v.at(10) threw std::out_of_range\n";
    }
}
