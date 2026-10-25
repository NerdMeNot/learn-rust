#include <stdio.h>
#include <stdlib.h>

void print_ints(const int *xs, size_t len) {
    printf("[");
    for (size_t i = 0; i < len; i++) {
        printf(i ? ", %d" : "%d", xs[i]);
    }
    printf("]\n");
}

int main(void) {
    int *a = malloc(3 * sizeof(int));
    a[0] = 1; a[1] = 2; a[2] = 3;
    // C has no moves: b is a copy of the pointer, so a and b share one buffer.
    // By convention b is now the owner, and we stop using a.
    int *b = a;
    a = NULL;
    print_ints(b, 3);

    int x = 5;
    int y = x;   // numbers are copied
    printf("%d %d\n", x, y);

    free(b);     // the owner frees it, exactly once
    return 0;
}
