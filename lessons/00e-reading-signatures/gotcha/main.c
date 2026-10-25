#include <stdio.h>
#include <string.h>

/* C has no generics: one function per type. */
static int largest_int(const int *list, int n) {
    int best = list[0];
    for (int i = 0; i < n; i++) {
        if (list[i] > best) {
            best = list[i];
        }
    }
    return best;
}

static const char *largest_str(const char **list, int n) {
    const char *best = list[0];
    for (int i = 0; i < n; i++) {
        if (strcmp(list[i], best) > 0) {
            best = list[i];
        }
    }
    return best;
}

int main(void) {
    int numbers[] = {3, 9, 4};
    const char *words[] = {"pear", "fig", "apple"};
    printf("largest number: %d\n", largest_int(numbers, 3));
    printf("largest word: %s\n", largest_str(words, 3));
    return 0;
}
