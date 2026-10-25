#include <stdio.h>

// C has no closures. A function cannot capture local variables,
// so you pass what it needs as extra arguments.
int times(int x, int factor) {
    return x * factor;
}

void inc(int *count) {   // a pointer lets it change the caller's variable
    *count += 1;
}

int main(void) {
    int factor = 3;
    printf("%d\n", times(5, factor));

    int count = 0;
    inc(&count);
    inc(&count);
    printf("count=%d\n", count);
    return 0;
}
