#include <iostream>
#include <vector>

int main() {
    std::vector<int> v;
    std::size_t last = v.capacity();
    std::cout << "start: capacity " << last << "\n";
    for (int i = 1; i <= 40; i++) {
        v.push_back(i);
        if (v.capacity() != last) {
            std::cout << "push_back #" << i << ": capacity " << last << " -> " << v.capacity() << "\n";
            last = v.capacity();
        }
    }

    std::vector<int> w{1, 2, 3};
    w.reserve(5);
    std::cout << "{1, 2, 3}.reserve(5): capacity " << w.capacity() << "\n";
}
