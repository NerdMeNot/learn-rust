#include <stdio.h>

int main(void) {
    int nums[] = {1, 2, 3, 4, 5, 6};

    // C has no map/filter: one loop does the filter and the map
    int evens_squared[6];
    int count = 0;
    for (int i = 0; i < 6; i++) {
        if (nums[i] % 2 == 0) {
            evens_squared[count++] = nums[i] * nums[i];
        }
    }

    int total = 0;
    for (int i = 0; i < 6; i++) {
        total += nums[i];
    }

    printf("[");
    for (int i = 0; i < count; i++) {
        printf(i ? ", %d" : "%d", evens_squared[i]);
    }
    printf("] %d\n", total);
    return 0;
}
