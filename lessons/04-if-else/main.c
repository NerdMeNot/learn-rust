#include <stdio.h>

int main(void) {
    int n = 7;
    if (n % 2 == 0) {
        printf("%d is even\n", n);
    } else {
        printf("%d is odd\n", n);
    }

    // C's if does not give back a value; the ?: operator does
    const char *size = n > 5 ? "big" : "small";
    printf("%s\n", size);
    return 0;
}
