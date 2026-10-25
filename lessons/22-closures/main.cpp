#include <iostream>

int main() {
    int factor = 3;
    auto times = [factor](int x) { return x * factor; };   // copies factor in
    std::cout << times(5) << "\n";

    int count = 0;
    auto inc = [&count] { count += 1; };                    // & captures count by reference
    inc();
    inc();
    std::cout << "count=" << count << "\n";
}
