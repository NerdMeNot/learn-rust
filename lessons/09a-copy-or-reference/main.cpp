#include <iostream>
#include <vector>

void print(const char *label, const std::vector<int> &v) {
    std::cout << label << "[";
    for (size_t i = 0; i < v.size(); i++) {
        std::cout << (i ? ", " : "") << v[i];
    }
    std::cout << "]\n";
}

int main() {
    std::vector<int> ages{30, 40, 50};

    // A copy: `auto` makes age a new int.
    for (auto age : ages) {
        age++;
        std::cout << "the copy is now " << age << "\n";
    }
    print("ages after the copy loop:     ", ages);

    // The elements themselves: `auto&` is a reference to each one.
    for (auto &age : ages) {
        age++;
    }
    print("ages after the in-place loop: ", ages);
    return 0;
}
