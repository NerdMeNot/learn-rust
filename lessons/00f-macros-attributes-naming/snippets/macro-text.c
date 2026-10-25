#include <stdio.h>

#define SQUARE(x) x * x
#define DOUBLE_INTO(dst, src) \
    {                         \
        int tmp = (src) * 2;  \
        dst = tmp;            \
    }

int main(void) {
    printf("SQUARE(1 + 2) = %d\n", SQUARE(1 + 2));
    int tmp = 0;
    printf("tmp before: %d\n", tmp);
    DOUBLE_INTO(tmp, 5);
    printf("tmp after DOUBLE_INTO(tmp, 5) = %d\n", tmp);
    return 0;
}
