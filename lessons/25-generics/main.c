#include <stdio.h>
#include <string.h>

// C has no generics: you write one version per type.
const int *largest_int(const int *items, size_t len) {
    const int *best = &items[0];
    for (size_t i = 0; i < len; i++) {
        if (items[i] > *best) best = &items[i];
    }
    return best;
}

const char *const *largest_str(const char *const *items, size_t len) {
    const char *const *best = &items[0];
    for (size_t i = 0; i < len; i++) {
        if (strcmp(items[i], *best) > 0) best = &items[i];
    }
    return best;
}

int main(void) {
    int nums[] = {3, 9, 2};
    const char *fruits[] = {"kiwi", "apple", "pear"};
    printf("%d\n", *largest_int(nums, 3));
    printf("%s\n", *largest_str(fruits, 3));
    return 0;
}
