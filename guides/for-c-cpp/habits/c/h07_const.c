#include <stdio.h>

int main(void) {
    int count = 1;      /* mutable unless you write const */
    count = count + 1;
    printf("count = %d\n", count);
    return 0;
}
