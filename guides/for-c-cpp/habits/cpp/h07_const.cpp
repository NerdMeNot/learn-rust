#include <iostream>

int main() {
    auto count = 1;     // mutable unless you write const
    count = count + 1;
    std::cout << "count = " << count << "\n";
    return 0;
}
