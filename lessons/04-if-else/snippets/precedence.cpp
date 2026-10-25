#include <iostream>

int main() {
    int x = 4;
    // same as C: `==` binds tighter than `&`, so this reads x & (1 == 0)
    if (x & 1 == 0) {
        std::cout << x << " is even\n";
    } else {
        std::cout << x << " is odd\n";
    }
    return 0;
}
