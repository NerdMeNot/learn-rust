#include <stdio.h>

int main(void) {
    int count = 0;
    count += 1; /* fine: mutable unless declared `const` */
    const int fixed = 0;
    fixed += 1;
    printf("%d\n", count);
    return 0;
}
