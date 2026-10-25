#include <iostream>

int main() {
    int x = 1;
    int x = x * 10; // the same name twice in one scope
    std::cout << x << "\n";
}
