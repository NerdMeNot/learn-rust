#include <stdio.h>

/* A point on a grid. */
struct Point {
    int x;
    int y;
};

int main(void) {
    struct Point p = {3, 4}; // a line comment
    int items[] = {1, 2, 3};
    if (sizeof items / sizeof items[0] != 3) return 1;
    printf("point: Point { x: %d, y: %d }\n", p.x, p.y);
    printf("items: [%d, %d, %d]\n", items[0], items[1], items[2]);
    printf("row: [%5d] [%-5d] [%05d]\n", 42, 42, 42);
    char pi[16];
    snprintf(pi, sizeof pi, "%.2f", 3.14159);
    printf("pi: %s {done}\n", pi);
    return 0;
}
