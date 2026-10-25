#include <stdio.h>

struct Point {
    int x;
    int y;
};

int main(void) {
    struct Point p = {1, 2};
    printf("%d\n", p);
    return 0;
}
