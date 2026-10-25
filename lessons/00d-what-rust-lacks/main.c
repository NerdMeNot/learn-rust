#include <stdio.h>

// What other languages write with ?:, i++, null and switch, written in C.
const char *size(int n) {
    switch (n) {
    case 1:
    case 2:
        return "small";
    case 3:
        return "medium";
    default:
        return "large";
    }
}

// Returns a pointer to the first even number, or NULL when there is none.
const int *first_even(const int *items, int len) {
    for (int i = 0; i < len; i++) {
        if (items[i] % 2 == 0) {
            return &items[i];
        }
    }
    return NULL;
}

int main(void) {
    int total = 0;
    for (int i = 0; i < 4; i++) {
        total += i;
    }
    const char *label = total > 5 ? "big" : "small";
    printf("total: %d (%s)\n", total, label);
    printf("size: %s %s\n", size(2), size(9));
    int a[] = {3, 5, 8};
    const int *even = first_even(a, 3);
    if (even != NULL) {
        printf("first even: %d\n", *even);
    } else {
        printf("no even number\n");
    }
    int b[] = {1, 3};
    const int *none = first_even(b, 2);
    printf("none: %d\n", none != NULL ? *none : -1);
    long long wide = (long long)total * 1000000000LL;
    printf("wide: %lld\n", wide);
    return 0;
}
