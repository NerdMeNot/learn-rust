#include <iostream>
#include <tuple>

int main() {
    std::tuple<int, int> t{3, 4};
    std::cout << t[0] << "\n";
}
