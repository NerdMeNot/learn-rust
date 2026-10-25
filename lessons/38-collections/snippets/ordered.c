#include <stdio.h>
#include <stdlib.h>

static int cmp(const void *a, const void *b) {
    int x = *(const int *)a, y = *(const int *)b;
    return (x > y) - (x < y);
}

int main(void) {
    int keys[] = {40, 10, 70, 25, 55};
    size_t n = sizeof keys / sizeof keys[0];
    qsort(keys, n, sizeof keys[0], cmp);  /* the "sorted map" is a sorted array */
    int want = 25;
    int *hit = bsearch(&want, keys, n, sizeof keys[0], cmp);
    printf("sorted:");
    for (size_t i = 0; i < n; i++) printf(" %d", keys[i]);
    printf("\nfound 25 at index %ld\n", hit ? (long)(hit - keys) : -1L);
    return 0;
}
