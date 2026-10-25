#include <stdio.h>

#define PRICE 10

/* static: this name is private to this file */
static int discount_percent(int qty) {
    return qty >= 10 ? 20 : 0;
}

int total(int qty) {
    int full = qty * PRICE;
    return full - full * discount_percent(qty) / 100;
}

int main(void) {
    printf("3 items: %d\n", total(3));
    printf("12 items: %d\n", total(12));
}
