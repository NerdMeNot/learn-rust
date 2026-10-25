#include <stdio.h>

// A C "closure": a function pointer plus a void * that carries the captured data.
typedef struct {
    int total;
} Sum;

static void add(void *ctx, int x) {
    Sum *s = ctx;   // the cast is the programmer's promise
    s->total += x;
}

static void for_each(const int *xs, int n, void (*f)(void *, int), void *ctx) {
    for (int i = 0; i < n; i++) f(ctx, xs[i]);
}

int main(void) {
    int xs[] = {1, 2, 3};
    Sum s = {0};
    for_each(xs, 3, add, &s);
    printf("total: %d\n", s.total);
    return 0;
}
