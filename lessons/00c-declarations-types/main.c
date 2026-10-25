#include <stdio.h>
#include <stdlib.h>

#define LIMIT 1000u

static double half(double x) {
    return x / 2.0;
}

int main(void) {
    const char *name = "Ada";
    unsigned char visits = 0;
    visits += 1;
    int mask = 0xff;
    float ratio = 2.5f;
    char letter = 'R';
    double total = (double)LIMIT + half(5.0);
    int parsed = atoi("42");

    printf("name=%s visits=%d\n", name, visits);
    printf("mask=%d ratio=%.1f letter=%c\n", mask, ratio, letter);
    printf("limit=%u total=%.1f\n", LIMIT, total);
    printf("parsed=%d\n", parsed);
    return 0;
}
