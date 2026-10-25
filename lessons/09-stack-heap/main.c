#include <stdio.h>

struct Point {
    int x, y;
};

int main(void) {
    struct Point a = {1, 2};
    struct Point b = a; // a struct is a value: b is a copy
    b.x = 99;
    printf("a = (%d, %d)\n", a.x, a.y);
    printf("b = (%d, %d)\n", b.x, b.y);

    // To share, you take an address on purpose.
    struct Point *p = &a;
    p->x = 50;
    printf("a after p->x = 50: (%d, %d)\n", a.x, a.y);
    return 0;
}
