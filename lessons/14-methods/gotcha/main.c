#include <stdio.h>

struct Counter {
    unsigned count;
};

void counter_bump(struct Counter *c) {
    c->count += 1;
}

int main(void) {
    const struct Counter c = {0};
    counter_bump(&c);
    printf("%u\n", c.count);
    return 0;
}
