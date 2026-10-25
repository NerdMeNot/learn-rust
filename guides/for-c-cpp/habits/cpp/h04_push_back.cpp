#include <iostream>
#include <vector>

int main() {
    std::vector<int> v{1, 2, 3};
    int &first = v[0];
    v.push_back(4);         // may reallocate
    std::cout << first << "\n";
    return 0;
}
