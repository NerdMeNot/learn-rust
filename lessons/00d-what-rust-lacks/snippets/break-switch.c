#include <stdio.h>

int main(void) {
    int n = 0;
    for (int i = 0; i < 5; i++) {
        switch (i) {
        case 2:
            break;
        default:
            n++;
        }
    }
    printf("n: %d\n", n);
    return 0;
}
