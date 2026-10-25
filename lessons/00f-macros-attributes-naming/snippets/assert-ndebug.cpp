#define NDEBUG
#include <cassert>
#include <iostream>

int main() {
    assert(1 + 1 == 3);
    std::cout << "assert did not stop the program\n";
    return 0;
}
