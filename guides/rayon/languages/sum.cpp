#include <iostream>

int main() {
    long long total = 0;
    #pragma omp parallel for reduction(+ : total)
    for (long long i = 1; i <= 1000000; i++) {
        total += i * i;
    }
    std::cout << total << "\n";
}
