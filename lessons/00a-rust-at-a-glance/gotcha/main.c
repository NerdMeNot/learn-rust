#include <stdio.h>

int main(void) {
    int total = 0;
    for (int n = 1; n <= 4; n++) {
        total += n;
    }
    printf("total: %d\n", total);
    return 0;
}
