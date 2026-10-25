#include <stdio.h>

// C has no generics. A macro pastes the text, and the compiler checks the result.
#define ADD(a, b) ((a) + (b))

int main(void) {
    printf("%d\n", ADD(2, 3));
    printf("%g\n", ADD(1.5, 2.25));
    return 0;
}
