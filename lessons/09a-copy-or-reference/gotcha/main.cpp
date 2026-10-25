#include <iostream>
#include <vector>

int main() {
    std::vector<int> ages{30, 40, 50};
    for (auto age : ages) {
        age += 1;
    }
    std::cout << "[" << ages[0] << ", " << ages[1] << ", " << ages[2] << "]\n";
    return 0;
}
