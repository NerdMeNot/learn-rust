#include <stdio.h>

int main(void) {
    int scores[] = {10, 20, 30};
    int *first = &scores[0];
    int *last = &scores[2];
    *first += 1;
    *last += 1;
    printf("[%d, %d, %d]\n", scores[0], scores[1], scores[2]);
    return 0;
}
