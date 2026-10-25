#include <stdio.h>
#include <stdlib.h>

// What Rc does, written by hand: a count next to the value, and a free when it hits zero.
struct Shared {
    int count;
    int value;
};

struct Shared *shared_new(int value) {
    struct Shared *s = malloc(sizeof *s);
    s->count = 1;
    s->value = value;
    return s;
}

struct Shared *shared_clone(struct Shared *s) {
    s->count++;
    return s;
}

void shared_drop(struct Shared *s) {
    s->count--;
    if (s->count == 0) {
        printf("count is zero: freeing\n");
        free(s);
    }
}

int main(void) {
    struct Shared *a = shared_new(7);
    struct Shared *b = shared_clone(a);
    printf("owners: %d\n", a->count);
    shared_drop(b);
    printf("owners: %d\n", a->count);
    shared_drop(a);
    return 0;
}
