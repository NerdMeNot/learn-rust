#include <stdio.h>

// const int *: read through the pointer, don't change or free it
int total(const int *xs, size_t len) {
    int sum = 0;
    for (size_t i = 0; i < len; i++) {
        sum += xs[i];
    }
    return sum;
}

int main(void) {
    int nums[] = {1, 2, 3};
    int t = total(nums, 3);   // lend nums to total
    printf("%d [%d, %d, %d]\n", t, nums[0], nums[1], nums[2]);   // still ours
    return 0;
}
