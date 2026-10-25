#include <stdint.h>
#include <stdio.h>

int main(void) {
    int runs = 0;
    /* a uint8_t is never above 255, so b <= 255 is always true; the guard stops it */
    for (uint8_t b = 0; b <= 255 && runs < 300; b++) {
        runs++;
    }
    printf("runs: %d (stopped by the guard)\n", runs);
    return 0;
}
