#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// C has no map type and no macro-free literal for one: an array of pairs
struct Entry {
    const char *name;
    int age;
};

static int by_name(const void *a, const void *b) {
    return strcmp(((const struct Entry *)a)->name, ((const struct Entry *)b)->name);
}

int main(void) {
    struct Entry ages[] = {{"Ada", 36}, {"Linus", 28}, {"Grace", 45}};
    size_t count = sizeof ages / sizeof ages[0];

    qsort(ages, count, sizeof ages[0], by_name);
    for (size_t i = 0; i < count; i++) {
        printf("%s: %d\n", ages[i].name, ages[i].age);
    }
    printf("entries: %zu\n", count);
    return 0;
}
