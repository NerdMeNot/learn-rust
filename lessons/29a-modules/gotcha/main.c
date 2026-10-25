#include <stdio.h>

static int discount_percent(int qty) {
    return qty >= 10 ? 20 : 0;
}

int total(int qty) {
    int full = qty * 10;
    return full - full * discount_percent(qty) / 100;
}

int main(void) {
    printf("total: %d\n", total(12));
    printf("discount: %d%%\n", discount_percent(12));
}
