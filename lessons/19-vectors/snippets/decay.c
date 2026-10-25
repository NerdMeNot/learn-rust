#include <stdio.h>

void show(int *p) {   // an array argument arrives as a plain pointer
    printf("inside show: sizeof p = %zu\n", sizeof p);
}

int main(void) {
    int a[4] = {10, 20, 30, 40};
    printf("in main: sizeof a = %zu bytes\n", sizeof a);
    show(a);
    return 0;
}
