#include <stdio.h>

int main(void) {
    int items[] = {5, 6, 7};
    size_t i = sizeof items / sizeof items[0] - 1;
    int guard = 0;
    /* size_t is never negative, so i >= 0 is always true; the guard stops it */
    while (i >= 0 && guard < 5) {
        printf("i = %zu\n", i);
        i--;
        guard++;
    }
    return 0;
}
