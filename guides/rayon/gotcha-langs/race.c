#include <stdio.h>

int main(void) {
    long long total = 0; /* shared by every thread */
    #pragma omp parallel for
    for (long long i = 1; i <= 1000000; i++) {
        total += i;
    }
    printf("total: %lld expected: 500000500000\n", total);
    return 0;
}
