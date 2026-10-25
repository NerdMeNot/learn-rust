#include <stdio.h>

int total(int *xs, size_t len) {
    int sum = 0;
    for (size_t i = 0; i < len; i++) {
        sum += xs[i];
    }
    return sum;
}

int main(void) {
    int nums[] = {1, 2, 3};
    int t = total(nums, 3);
    printf("%d [%d, %d, %d]\n", t, nums[0], nums[1], nums[2]);
    return 0;
}
