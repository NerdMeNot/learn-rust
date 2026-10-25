#include <stdio.h>
#include <stdlib.h>

int main(void) {
    int *a = malloc(3 * sizeof *a);
    a[0] = 1;
    int *first = &a[0];
    a = realloc(a, 1000 * sizeof *a);   /* may move the block */
    printf("%d\n", *first);
    free(a);
    return 0;
}
