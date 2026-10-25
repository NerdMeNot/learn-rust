#include <iostream>
#include <string>

template <typename T>
std::string describe(T) { return "something"; }

template <>
std::string describe<int>(int) { return "an integer"; }

int main() {
    std::cout << describe(5) << ", " << describe(2.5) << "\n";
    return 0;
}
