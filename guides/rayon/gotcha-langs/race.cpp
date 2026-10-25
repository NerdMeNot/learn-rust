#include <iostream>

int main() {
    long long total = 0; // shared by every thread
    #pragma omp parallel for
    for (long long i = 1; i <= 1000000; i++) {
        total += i;
    }
    std::cout << "total: " << total << " expected: 500000500000\n";
}
