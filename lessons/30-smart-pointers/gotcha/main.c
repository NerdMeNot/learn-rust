#include <stdio.h>
#include <stdlib.h>

struct Counter {
    int hits;
};

int main(void) {
    struct Counter *a = calloc(1, sizeof *a);
    struct Counter *b = a;

    a->hits++;
    b->hits++;
    printf("hits: %d\n", a->hits);

    free(a);
    return 0;
}
