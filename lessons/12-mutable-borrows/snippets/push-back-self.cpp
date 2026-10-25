#include <iostream>
#include <vector>

int main() {
    std::vector<int> v{1, 2, 3, 4};
    const int* before = v.data();
    v.push_back(v[0]);   // v[0] lives in the buffer that push_back is about to move
    std::cout << "buffer moved: " << std::boolalpha << (v.data() != before) << "\n";
    std::cout << "last element: " << v.back() << "\n";
}
