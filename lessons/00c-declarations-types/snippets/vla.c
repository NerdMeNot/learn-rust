#include <stdio.h>

int main(void) {
    int n = 3;
    int grid[n]; // a variable-length array: the size is read at run time
    printf("elements: %zu\n", sizeof(grid) / sizeof(grid[0]));
    return 0;
}
