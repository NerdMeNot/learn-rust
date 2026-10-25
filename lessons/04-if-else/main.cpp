#include <iostream>
#include <string_view>

int main() {
    int n = 7;
    if (n % 2 == 0) {
        std::cout << n << " is even\n";
    } else {
        std::cout << n << " is odd\n";
    }

    // the ternary operator gives back a value
    std::string_view size = n > 5 ? "big" : "small";
    std::cout << size << "\n";
}
