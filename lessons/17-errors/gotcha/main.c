#include <stdio.h>
#include <stdlib.h>

int main(void) {
    int n = atoi("42");
    int bad = atoi("abc");
    printf("%d %d\n", n + 1, bad + 1);
    return 0;
}
