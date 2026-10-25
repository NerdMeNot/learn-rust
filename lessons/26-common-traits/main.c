#include <stdio.h>
#include <stdlib.h>

// C has no traits: no ==, no printing and no ordering for a struct.
// Each one is a function you write and call by name.
typedef struct {
    unsigned major;
    unsigned minor;
} Version;

int version_cmp(const void *pa, const void *pb) {
    const Version *a = pa, *b = pb;
    if (a->major != b->major) return a->major < b->major ? -1 : 1;
    if (a->minor != b->minor) return a->minor < b->minor ? -1 : 1;
    return 0;
}

int main(void) {
    Version a = {1, 10};
    Version b = {1, 9};
    printf("display: v%u.%u\n", a.major, a.minor);
    printf("a == b: %s\n", version_cmp(&a, &b) == 0 ? "true" : "false");
    printf("a > b: %s\n", version_cmp(&a, &b) > 0 ? "true" : "false");

    Version all[3] = {a, b, {0, 0}};
    qsort(all, 3, sizeof(Version), version_cmp);
    printf("sorted:");
    for (int i = 0; i < 3; i++) printf(" v%u.%u", all[i].major, all[i].minor);
    printf("\n");

    // No hash set in the standard library: count distinct values by comparing.
    Version list[4] = {a, b, {0, 0}, a};
    int distinct = 0;
    for (int i = 0; i < 4; i++) {
        int dup = 0;
        for (int j = 0; j < i; j++) {
            if (version_cmp(&list[i], &list[j]) == 0) dup = 1;
        }
        if (!dup) distinct++;
    }
    printf("distinct: %d\n", distinct);
    return 0;
}
