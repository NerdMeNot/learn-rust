#include <stdio.h>

int main(void) {
    const char *names[] = {"ana", "bo"};
    size_t names_len = sizeof names / sizeof names[0];

    for (size_t i = 0; i < names_len; i++) {
        printf("hi %s\n", names[i]);
    }

    int nums[] = {1, 2, 3};
    for (size_t i = 0; i < 3; i++) {
        nums[i] *= 10;          // change each item through its index
    }

    for (size_t i = 0; i < 3; i++) {
        printf("%zu: %d\n", i, nums[i]);
    }
    printf("%zu names, [%d, %d, %d]\n", names_len, nums[0], nums[1], nums[2]);
    return 0;
}
