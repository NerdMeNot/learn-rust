#include <stdio.h>

int main(void) {
    int prices[] = {10, 20, 30};
    int *p = prices;
    int second = *(p + 1);
    printf("second: %d\n", second);
    return 0;
}
