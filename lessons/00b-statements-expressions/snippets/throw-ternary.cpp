#include <iostream>
#include <stdexcept>

int main() {
    bool ok = true;
    int x = ok ? 5 : throw std::runtime_error("not ok");
    std::cout << "x: " << x << "\n";
}
