#include <stdio.h>
#include <stdlib.h>

int main(void) {
    int n = atoi("abc");        /* no way to tell "0" from "not a number" */
    printf("n = %d\n", n);
    return 0;
}
