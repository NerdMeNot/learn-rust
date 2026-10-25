#include <stdio.h>
#include <stdarg.h>
#include <stdlib.h>

static int twice(int x);

static void divmod(int a, int b, int *q, int *r) {
    *q = a / b;
    *r = a % b;
}

static int apply(int (*f)(int), int x) {
    return f(x);
}

static int twice(int x) {
    return x * 2;
}

static int sum_n(int count, ...) {
    va_list ap;
    va_start(ap, count);
    int total = 0;
    for (int i = 0; i < count; i++) total += va_arg(ap, int);
    va_end(ap);
    return total;
}

int main(void) {
    int q, r;
    divmod(7, 2, &q, &r);
    if (q != 3 || r != 1) abort();
    if (apply(twice, 4) != 8) abort();
    if (sum_n(3, 1, 2, 3) != 6) abort();
    puts("all checks passed");
    return 0;
}
