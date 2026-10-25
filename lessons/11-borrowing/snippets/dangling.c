#include <stdio.h>

int main(void) {
    int *r;
    {
        int x = 5;
        r = &x;
    }
    printf("*r = %d\n", *r); // x is out of scope here
    return 0;
}
