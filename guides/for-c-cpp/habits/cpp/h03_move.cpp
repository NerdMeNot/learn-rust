#include <iostream>
#include <utility>
#include <vector>

int main() {
    std::vector<int> a{1, 2, 3};
    std::vector<int> b = std::move(a);
    std::cout << "a.size() = " << a.size() << ", b.size() = " << b.size() << "\n";
    a.push_back(9);     // allowed: a is valid, its contents are unspecified
    std::cout << "a.size() = " << a.size() << "\n";
    return 0;
}
