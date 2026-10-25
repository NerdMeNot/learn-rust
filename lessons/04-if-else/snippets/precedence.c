#include <stdio.h>

int main(void) {
    int x = 4;
    /* in C `==` binds tighter than `&`, so this reads x & (1 == 0), which is x & 0 */
    if (x & 1 == 0) {
        printf("%d is even\n", x);
    } else {
        printf("%d is odd\n", x);
    }
    return 0;
}
