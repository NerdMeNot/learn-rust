#include <iostream>

int main() {
    int total = 0;
    for (int n = 1; n <= 4; n++) {
        total += n;
    }
    std::cout << "total: " << total << "\n";
}
