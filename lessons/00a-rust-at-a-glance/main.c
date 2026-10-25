#include <stdio.h>

// Greet someone, then add up 1 to 4.
void greet(const char *name) {
    printf("Hello, %s!\n", name);
}

int add(int a, int b) {
    return a + b;
}

int main(void) {
    greet("Ferris");
    int total = 0;
    for (int n = 1; n <= 4; n++) {
        total = add(total, n);
    }
    printf("total: %d\n", total);
    return 0;
}
