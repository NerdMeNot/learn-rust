#include <iostream>
#include <string>

// Greet someone, then add up 1 to 4.
std::string greet(const std::string& name) {
    return "Hello, " + name + "!";
}

int add(int a, int b) {
    return a + b;
}

int main() {
    std::cout << greet("Ferris") << "\n";
    int total = 0;
    for (int n = 1; n <= 4; n++) {
        total = add(total, n);
    }
    std::cout << "total: " << total << "\n";
}
