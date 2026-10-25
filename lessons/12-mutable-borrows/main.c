#include <stdio.h>
#include <stdlib.h>

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

// a non-const pointer: this function may change the list
void add_bonus(struct IntVec *scores, int bonus) {
    for (size_t i = 0; i < scores->len; i++) {
        scores->data[i] += bonus;
    }
    push(scores, 0);
}

int main(void) {
    struct IntVec scores = {0};
    push(&scores, 10);
    push(&scores, 20);
    add_bonus(&scores, 5);

    printf("[");
    for (size_t i = 0; i < scores.len; i++) {
        printf(i ? ", %d" : "%d", scores.data[i]);
    }
    printf("]\n");
    free(scores.data);
    return 0;
}
