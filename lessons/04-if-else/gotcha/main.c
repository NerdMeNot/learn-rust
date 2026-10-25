#include <stdio.h>

int main(void) {
    int n = 7;
    const char *size = n > 5 ? "big" : 0;
    printf("%s\n", size);
    return 0;
}
