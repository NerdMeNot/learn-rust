#include <iostream>
#include <vector>

void print(const std::vector<int>& xs) {
    std::cout << "[";
    for (std::size_t i = 0; i < xs.size(); ++i) {
        std::cout << (i ? ", " : "") << xs[i];
    }
    std::cout << "]";
}

int main() {
    std::vector<int> a{1, 2, 3};
    auto b = a;  // copies the whole vector
    print(a);
    std::cout << " ";
    print(b);
    std::cout << "\n";
}
