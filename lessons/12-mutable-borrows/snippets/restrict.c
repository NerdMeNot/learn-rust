#include <stdio.h>

// Plain pointers: the function must be correct even when both point at one int.
void add_twice(int *total, const int *amount) {
    *total += *amount;
    *total += *amount;
}

// `restrict` promises that nothing else reaches *total while this function runs.
// The optimiser believes it, so it may read *amount once. GCC's default is no
// optimisation, so ask for it on this one function to make the effect visible.
__attribute__((optimize("O2")))
void add_twice_restrict(int *restrict total, const int *restrict amount) {
    *total += *amount;
    *total += *amount;
}

int main(void) {
    int x = 1, y = 1;
    add_twice(&x, &y);
    printf("plain, separate:    x = %d\n", x);
    x = 1;
    add_twice(&x, &x);
    printf("plain, same int:    x = %d\n", x);

    x = 1;
    add_twice_restrict(&x, &y);
    printf("restrict, separate: x = %d\n", x);
    x = 1;
    add_twice_restrict(&x, &x); // breaks the promise: undefined behaviour
    printf("restrict, same int: x = %d\n", x);
    return 0;
}
