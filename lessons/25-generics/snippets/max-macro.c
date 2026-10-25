#include <stdio.h>

// One "generic" max, by pasting text. It works for int and for double.
#define MAX(a, b) ((a) > (b) ? (a) : (b))

int main(void) {
    printf("MAX(3, 9) = %d\n", MAX(3, 9));
    printf("MAX(2.5, 1.5) = %g\n", MAX(2.5, 1.5));

    int i = 5;
    int j = 3;
    int bigger = MAX(i++, j);
    printf("MAX(i++, j) = %d, and i is now %d\n", bigger, i);
    return 0;
}
