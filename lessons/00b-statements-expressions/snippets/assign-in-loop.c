#include <stdio.h>

int main(void) {
    const char *p = "abc";
    int c;
    while ((c = *p++) != '\0') {
        printf("char: %c\n", c);
    }
    return 0;
}
