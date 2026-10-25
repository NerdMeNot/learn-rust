#include <stdio.h>

int main(void) {
    int x = 1;
    {
        int x = 10; // allowed: an inner block
        printf("inner x: %d\n", x);
    }
    printf("outer x: %d\n", x);
    return 0;
}
