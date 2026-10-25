#include <limits.h>
#include <stdint.h>
#include <stdio.h>

int main(int argc, char **argv) {
    (void)argv;
    int x = -1;
    unsigned int y = 1;
    printf("-1 < 1u is %s\n", x < y ? "true" : "false");

    uint8_t a = 200, b = 100;
    uint8_t c = a + b;                   /* a + b is an int: 300, cut to 8 bits */
    printf("200 + 100 = %d, stored in 8 bits: %d\n", a + b, c);

    int big = INT_MAX - 1 + argc;        /* INT_MAX */
    printf("INT_MAX + 1 = %d\n", big + 1);
    return 0;
}
