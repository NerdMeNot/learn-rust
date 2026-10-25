#include <stdio.h>

int main(void) {
    // 1, 2, 3 (the end is not included)
    for (int i = 1; i < 4; i++) {
        printf("for %d\n", i);
    }

    int n = 3;
    while (n > 0) {
        n -= 1;
    }

    // C's break cannot hand back a value, so set a variable first
    int tries = 0;
    int found;
    for (;;) {
        tries += 1;
        if (tries == 3) {
            found = tries * 10;
            break;
        }
    }
    printf("n=%d found=%d\n", n, found);
    return 0;
}
