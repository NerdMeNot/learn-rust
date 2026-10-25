#include <iostream>
#include <utility>
#include <vector>

void print(const std::vector<int>& xs) {
    std::cout << "[";
    for (std::size_t i = 0; i < xs.size(); ++i) {
        std::cout << (i ? ", " : "") << xs[i];
    }
    std::cout << "]\n";
}

int main() {
    std::vector<int> a{1, 2, 3};
    std::vector<int> b = std::move(a);   // move the buffer to b. Don't use a after this.
    print(b);

    int x = 5;
    int y = x;                           // plain copy
    std::cout << x << " " << y << "\n";
}
