#include <iostream>
#include <memory>
#include <vector>

int main() {
    std::vector<int, std::allocator<int>> v;    // the allocator is a template parameter
    v.push_back(1);
    std::cout << v.size() << "\n";
    return 0;
}
