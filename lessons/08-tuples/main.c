#include <stdio.h>

// C has no tuples: return a small struct instead
struct MinMax {
    int lo;
    int hi;
};

struct MinMax min_max(const int *xs, size_t len) {
    struct MinMax r = {xs[0], xs[0]};
    for (size_t i = 0; i < len; i++) {
        if (xs[i] < r.lo) r.lo = xs[i];
        if (xs[i] > r.hi) r.hi = xs[i];
    }
    return r;
}

int main(void) {
    int a[] = {4, 9, 1, 7};
    struct MinMax r = min_max(a, 4);
    printf("%d %d\n", r.lo, r.hi);

    int b[] = {3, 5};
    printf("%d\n", min_max(b, 2).hi);   // only use the field you need
    return 0;
}
