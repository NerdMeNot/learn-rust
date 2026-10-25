#include <iostream>
#include <string>

int main() {
    int n = std::stoi("abc");   // throws; nothing in the type says so
    std::cout << "n = " << n << "\n";
    return 0;
}
