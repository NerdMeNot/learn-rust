#include <iostream>
#include <span>
#include <vector>

void print(std::span<const int> xs) {
    std::cout << "[";
    for (std::size_t i = 0; i < xs.size(); ++i) {
        std::cout << (i ? ", " : "") << xs[i];
    }
    std::cout << "]";
}

int main() {
    std::vector<int> v;
    v.push_back(10);
    v.push_back(20);

    std::vector<int> w{1, 2, 3};
    w.push_back(4);

    std::span<const int> part = std::span(w).subspan(1, 2);   // a view, no copy
    print(v);
    std::cout << " ";
    print(w);
    std::cout << " ";
    print(part);
    std::cout << " len=" << w.size() << "\n";
}
