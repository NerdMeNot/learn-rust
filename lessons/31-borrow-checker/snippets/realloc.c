#include <stdio.h>
#include <stdlib.h>

struct Vec {
    int *data;
    int len;
    int cap;
};

void push(struct Vec *v, int x) {
    if (v->len == v->cap) {
        v->cap *= 1000000;
        v->data = realloc(v->data, v->cap * sizeof(int));
    }
    v->data[v->len++] = x;
}

int main(void) {
    // Direct: the compiler sees the realloc and the later use.
    int *a = malloc(4 * sizeof(int));
    a[0] = 1;
    int *first = &a[0];
    a = realloc(a, 1000000 * sizeof(int));
    printf("direct: %d\n", *first);
    free(a);

    // Through a helper: the same bug, and no warning.
    struct Vec v = {malloc(4 * sizeof(int)), 0, 4};
    for (int i = 0; i < 4; i++) push(&v, i + 1);
    int *head = &v.data[0];
    push(&v, 5);
    printf("helper: %d\n", *head);
    free(v.data);
}
