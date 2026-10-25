#include <stdio.h>

static void get(const int *a, int len, int i) {
    if (i < len) printf("get(%d): Some(%d)\n", i, a[i]);
    else printf("get(%d): None\n", i);
}

int main(void) {
    int prices[] = {10, 20, 30};
    int *p = prices;
    int second = *(p + 1);
    printf("second, read by offset: %d\n", second);
    get(prices, 3, 1);
    get(prices, 3, 9);
    return 0;
}
