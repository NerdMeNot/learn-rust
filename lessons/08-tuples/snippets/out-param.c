#include <stdio.h>

// Two results, both written through pointers.
void divmod(int a, int b, int *q, int *r) {
    *q = a / b;
    *r = a % b;
}

int main(void) {
    int q, r;
    divmod(17, 5, &q, &r);
    printf("q: %d, r: %d\n", q, r);
    return 0;
}
