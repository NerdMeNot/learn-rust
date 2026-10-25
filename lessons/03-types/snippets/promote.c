#include <stdint.h>
#include <stdio.h>

int main(void) {
    uint8_t a = 200, b = 100;
    printf("sizeof(a + b): %zu\n", sizeof(a + b));
    printf("a + b: %d\n", a + b);
    uint8_t c = a + b;
    printf("stored in a uint8_t: %d\n", c);

    int neg = -1;
    unsigned one = 1;
    printf("-1 < 1u: %d\n", neg < one);

    uint8_t sum;
    int wrapped = __builtin_add_overflow(a, b, &sum);
    printf("__builtin_add_overflow: %d, sum %d\n", wrapped, sum);

    char ch = (char)200;
    printf("(char)200: %d\n", ch);
    printf("sizeof(int): %zu, sizeof(long): %zu\n", sizeof(int), sizeof(long));
    return 0;
}
