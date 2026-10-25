#include <stdio.h>

struct Counter {
    unsigned count;
};

// C has no methods: plain functions that take the struct by pointer
struct Counter counter_new(void) {
    return (struct Counter){.count = 0};
}

unsigned counter_get(const struct Counter *c) {   // const: only reads
    return c->count;
}

void counter_bump(struct Counter *c) {            // no const: changes it
    c->count += 1;
}

int main(void) {
    struct Counter c = counter_new();
    counter_bump(&c);
    counter_bump(&c);
    printf("%u\n", counter_get(&c));
    return 0;
}
