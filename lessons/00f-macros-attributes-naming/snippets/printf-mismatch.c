#include <stdio.h>

int main(void) {
    /* %ld expects a long, but 5 is an int: undefined behaviour in the C standard. */
    printf("%ld\n", 5);
    return 0;
}
