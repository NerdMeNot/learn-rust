#include <stdio.h>
#include <stdlib.h>

int main(void) {
    int *prices = malloc(3 * sizeof(int));
    prices[0] = 10;
    prices[1] = 20;
    prices[2] = 30;
    printf("prices[3] = %d\n", prices[3]); /* one past the last element */
    free(prices);
    return 0;
}
