#include <iostream>

int main() {
    int a, b;
    a = b = 5;
    std::cout << "chain: " << a << " " << b << "\n";
    int x = 0;
    if (x = 1) {
        std::cout << "x assigned in the condition: " << x << "\n";
    }
}
