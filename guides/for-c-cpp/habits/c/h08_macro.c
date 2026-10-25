#include <stdio.h>

#define MAX(a, b) ((a) > (b) ? (a) : (b))

int main(void) {
    int i = 5, j = 3;
    int m = MAX(i++, j);    /* expands to ((i++) > (j) ? (i++) : (j)) */
    printf("m = %d, i = %d\n", m, i);
    return 0;
}
