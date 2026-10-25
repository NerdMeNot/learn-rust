#include <iostream>
#include <vector>

int main(int argc, char **argv) {
    std::vector<int> v{1, 2, 3};
    int i = static_cast<int>(v.size()) + 2;   // 5
    if (argc > 1 && argv[1][0] == 'a') {
        std::cout << v.at(i) << "\n";   // checked: throws
    } else {
        std::cout << v[i] << "\n";      // unchecked
    }
    return 0;
}
