#include <stdio.h>

int main(void) {
    int a, b;
    a = b = 5;
    printf("chain: %d %d\n", a, b);
    int x = 0;
    if (x = 1) {
        printf("x assigned in the condition: %d\n", x);
    }
    return 0;
}
