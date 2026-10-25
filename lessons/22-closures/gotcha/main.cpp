#include <iostream>

int main() {
    int count = 0;
    auto inc = [&] { count += 1; };
    inc();
    inc();
    std::cout << "count=" << count << "\n";
}
