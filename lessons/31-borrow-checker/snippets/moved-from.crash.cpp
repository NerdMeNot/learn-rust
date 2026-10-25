#include <iostream>
#include <vector>

int main() {
    std::vector<int> v = {1, 2, 3};
    std::vector<int> t = std::move(v);
    std::cout << "t.size(): " << t.size() << std::endl;
    std::cout << "v.size() after the move: " << v.size() << std::endl;
    std::cout << "v[0]: " << v[0] << std::endl;
}
