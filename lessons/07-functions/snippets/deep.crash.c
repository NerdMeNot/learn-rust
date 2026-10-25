#include <stdio.h>

long depth(long n) {
    return n == 0 ? 0 : 1 + depth(n - 1);
}

int main(void) {
    printf("depth(500): %ld\n", depth(500));
    fflush(stdout);
    printf("depth(100000000): %ld\n", depth(100000000));
    return 0;
}
