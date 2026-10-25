#include <iostream>
#include <vector>

int main() {
    std::vector<int> v{1, 2, 3};

    for (auto x : v) {
        x *= 10; // x is a copy
    }
    std::cout << "after auto x: " << v[0] << " " << v[1] << " " << v[2] << "\n";

    for (auto& x : v) {
        x *= 10; // x refers to the item
    }
    std::cout << "after auto& x: " << v[0] << " " << v[1] << " " << v[2] << "\n";
}
