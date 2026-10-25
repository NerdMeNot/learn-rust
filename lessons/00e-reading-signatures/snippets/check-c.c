#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

_Static_assert(sizeof(int) == 4, "int is 32 bits here");

static int max_int(int a, int b) { return b > a ? b : a; }

static int apply(int (*f)(int), int x) { return f(x); }

static int inc(int x) { return x + 1; }

static const int *find(const int *a, int n, int key) {
    for (int i = 0; i < n; i++) {
        if (a[i] == key) return &a[i];
    }
    return NULL;
}

static int read_file(const char *path, char *buf, size_t n) {
    FILE *f = fopen(path, "r");
    if (f == NULL) return -1;
    size_t got = fread(buf, 1, n - 1, f);
    buf[got] = '\0';
    fclose(f);
    return 0;
}

static void bump(int *x) { (*x)++; }

static double max_double(double a, double b) { return b > a ? b : a; }

/* C11 `_Generic` only picks between per-type functions by argument type */
#define MAX(a, b) _Generic((a), int: max_int, double: max_double)(a, b)

int main(void) {
    assert(MAX(1, 2) == 2);
    assert(MAX(1.5, 0.5) == 1.5);
    int xs[] = {1, 2};
    char buf[16];
    int n = 1;
    assert(max_int(1, 2) == 2);
    assert(apply(inc, 1) == 2);
    assert(find(xs, 2, 2) == &xs[1]);
    assert(find(xs, 2, 5) == NULL);
    assert(read_file("/nope", buf, sizeof buf) == -1);
    bump(&n);
    assert(n == 2);
    puts("c rows ok");
    return 0;
}
