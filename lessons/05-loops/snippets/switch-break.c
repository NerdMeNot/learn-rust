#include <stdio.h>

int main(void) {
    for (int n = 1; n <= 5; n++) {
        switch (n) {
        case 3:
            break; /* leaves the switch only */
        default:
            printf("n: %d\n", n);
        }
    }
    printf("after the loop\n");
    return 0;
}
