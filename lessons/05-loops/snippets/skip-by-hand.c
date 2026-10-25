#include <stdio.h>

int main(void) {
    int passes = 0;
    for (int i = 0; i < 5; i++) {
        if (i == 1) {
            i += 2;
        }
        passes++;
        printf("i: %d\n", i);
    }
    printf("passes: %d\n", passes);
    return 0;
}
