#include <iostream>

int main() {
    // 1, 2, 3 (the end is not included)
    for (int i = 1; i < 4; ++i) {
        std::cout << "for " << i << "\n";
    }

    int n = 3;
    while (n > 0) {
        n -= 1;
    }

    // break can't hand back a value, so a lambda that returns does the job
    int tries = 0;
    const int found = [&] {
        while (true) {
            tries += 1;
            if (tries == 3) {
                return tries * 10;
            }
        }
    }();
    std::cout << "n=" << n << " found=" << found << "\n";
}
