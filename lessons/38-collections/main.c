#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// C's std has no map, set, queue or heap: arrays plus qsort do the job here
struct Count {
    char word[16];
    int n;
};

int by_count(const void *a, const void *b) {
    const struct Count *x = a, *y = b;
    if (x->n != y->n) return y->n - x->n;
    return strcmp(x->word, y->word);
}

int by_word(const void *a, const void *b) {
    return strcmp(((const struct Count *)a)->word, ((const struct Count *)b)->word);
}

int by_int(const void *a, const void *b) {
    return *(const int *)a - *(const int *)b;
}

int main(void) {
    char text[] = "the cat saw the dog and the cat ran";

    // count words with a linear search (strtok writes into text, so it's an array)
    struct Count counts[16];
    int len = 0;
    for (char *w = strtok(text, " "); w; w = strtok(NULL, " ")) {
        int i = 0;
        while (i < len && strcmp(counts[i].word, w) != 0) i++;
        if (i == len) {
            snprintf(counts[len].word, sizeof counts[len].word, "%s", w);
            counts[len++].n = 0;
        }
        counts[i].n++;
    }
    qsort(counts, (size_t)len, sizeof counts[0], by_count);
    printf("top: %s=%d %s=%d %s=%d\n", counts[0].word, counts[0].n,
           counts[1].word, counts[1].n, counts[2].word, counts[2].n);

    // a "sorted map": sort the array by key
    struct Count by_name[] = {{"zoe", 31}, {"ana", 30}, {"mia", 28}};
    qsort(by_name, 3, sizeof by_name[0], by_word);
    printf("sorted: %s=%d %s=%d %s=%d\n", by_name[0].word, by_name[0].n,
           by_name[1].word, by_name[1].n, by_name[2].word, by_name[2].n);

    // the counts array already holds each word once, so it doubles as a set
    int has_dog = 0;
    for (int i = 0; i < len; i++) has_dog |= strcmp(counts[i].word, "dog") == 0;
    printf("%d unique words, has dog: %s\n", len, has_dog ? "true" : "false");

    // a queue: an array with a read index and a write index
    const char *queue[8] = {"a", "b"};
    int head = 0, tail = 2;
    queue[tail++] = "c";
    const char *first = queue[head++];
    printf("served %s, waiting %s %s\n", first, queue[head], queue[head + 1]);

    // no heap: sort once and read from the front
    int nums[] = {5, 1, 8, 3};
    qsort(nums, 4, sizeof nums[0], by_int);
    printf("smallest first:");
    for (int i = 0; i < 4; i++) printf(" %d", nums[i]);
    printf("\n");
    return 0;
}
