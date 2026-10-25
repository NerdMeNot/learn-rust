#include <stdio.h>

int main(int argc, char **argv) {
    (void)argv;
    int a[3] = {1, 2, 3};
    int i = argc + 4;       /* 5, but the compiler cannot see that */
    printf("%d\n", a[i]);
    return 0;
}
