#include <stdio.h>
#include <stdlib.h>

// C has no Vec: a growable array is a pointer, a length and a capacity
struct IntVec {
    int *data;
    size_t len, cap;
};

void push(struct IntVec *v, int x) {
    if (v->len == v->cap) {
        v->cap = v->cap ? v->cap * 2 : 4;
        v->data = realloc(v->data, v->cap * sizeof(int));
    }
    v->data[v->len++] = x;
}

void print_ints(const int *xs, size_t len) {
    printf("[");
    for (size_t i = 0; i < len; i++) {
        printf(i ? ", %d" : "%d", xs[i]);
    }
    printf("]");
}

int main(void) {
    struct IntVec v = {0};
    push(&v, 10);
    push(&v, 20);

    struct IntVec w = {0};
    for (int i = 1; i <= 4; i++) push(&w, i);

    const int *part = &w.data[1];   // a "slice": pointer + length, no copy
    size_t part_len = 2;

    print_ints(v.data, v.len);
    printf(" ");
    print_ints(w.data, w.len);
    printf(" ");
    print_ints(part, part_len);
    printf(" len=%zu\n", w.len);

    free(v.data);
    free(w.data);
    return 0;
}
