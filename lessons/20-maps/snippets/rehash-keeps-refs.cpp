#include <iostream>
#include <unordered_map>

int main() {
    std::unordered_map<int, int> m;
    m[0] = 1;
    int *before = &m[0];
    for (int i = 1; i < 10000; i++) m[i] = i;  // plenty of rehashing
    std::cout << "element still at the same address: " << (before == &m[0]) << "\n";
}
