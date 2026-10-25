#include <iostream>
#include <vector>

int main() {
    std::vector<int> v{10, 20, 30};
    int i = 10;
    std::cout << "v[" << i << "] is " << v[i] << "\n";   // no check
}
