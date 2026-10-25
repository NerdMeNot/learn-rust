#include <stdio.h>

struct Counter {
    unsigned count;
    void (*bump)(struct Counter *); // a method stored in the value
};

static void bump_impl(struct Counter *c) {
    c->count += 1;
}

int main(void) {
    struct Counter c = {0, bump_impl};
    c.bump(&c); // you pass the object yourself
    c.bump(&c);
    printf("count: %u\n", c.count);
    printf("sizeof(struct Counter): %zu\n", sizeof(struct Counter));
    return 0;
}
