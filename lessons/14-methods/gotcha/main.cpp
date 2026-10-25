#include <iostream>

struct Counter {
    unsigned count = 0;
    void bump() { count += 1; }
};

int main() {
    const Counter c;
    c.bump();
    std::cout << c.count << "\n";
}
