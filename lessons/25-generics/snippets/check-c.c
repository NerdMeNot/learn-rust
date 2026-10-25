#include <stdlib.h>
#include <stdio.h>

// Backs the C cells of the table in lesson.mdx. Never shown.
static int by_int(const void *a, const void *b) {
    return *(const int *)a - *(const int *)b;
}

int main(void) {
    int nums[] = {3, 1, 2};
    qsort(nums, 3, sizeof nums[0], by_int);
    if (nums[0] != 1 || nums[2] != 3) return 1;
    printf("ok\n");
    return 0;
}
