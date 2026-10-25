#include <stdio.h>
#include <stdlib.h>

int main(void) {
    int *a = malloc(3 * sizeof(int));
    a[0] = 1; a[1] = 2; a[2] = 3;
    int *b = a;
    printf("[%d, %d, %d] [%d, %d, %d]\n", a[0], a[1], a[2], b[0], b[1], b[2]);

    // clean up both variables
    free(a);
    free(b);
    return 0;
}
