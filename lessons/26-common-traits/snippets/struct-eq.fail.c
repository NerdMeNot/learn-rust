#include <stdio.h>

typedef struct {
    unsigned major;
    unsigned minor;
} Version;

int main(void) {
    Version a = {1, 2};
    Version b = {1, 2};
    printf("equal: %d\n", a == b);
    return 0;
}
