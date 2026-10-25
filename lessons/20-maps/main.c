#include <stdio.h>
#include <string.h>

// C has no map: a small array of key/value pairs, searched in order
struct Entry {
    const char *key;
    unsigned value;
};

struct Map {
    struct Entry items[16];
    int len;
};

// returns a pointer to the value, or NULL when the key is missing
unsigned *get(struct Map *m, const char *key) {
    for (int i = 0; i < m->len; i++) {
        if (strcmp(m->items[i].key, key) == 0) return &m->items[i].value;
    }
    return NULL;
}

void insert(struct Map *m, const char *key, unsigned value) {
    unsigned *v = get(m, key);
    if (v) {
        *v = value;
    } else {
        m->items[m->len++] = (struct Entry){key, value};
    }
}

int main(void) {
    struct Map ages = {0};
    insert(&ages, "ana", 30);
    insert(&ages, "bo", 25);

    unsigned *age = get(&ages, "ana");
    if (age) {
        printf("ana is %u\n", *age);
    } else {
        printf("no ana\n");
    }

    // a default on purpose: 0 when the key is missing
    unsigned *z = get(&ages, "zed");
    printf("zed is %u\n", z ? *z : 0);

    // count words: get it, or insert it first
    struct Map counts = {0};
    const char *words[] = {"a", "b", "a", "c", "a"};
    for (int i = 0; i < 5; i++) {
        if (!get(&counts, words[i])) insert(&counts, words[i], 0);
        *get(&counts, words[i]) += 1;
    }
    printf("a appears %u times\n", *get(&counts, "a"));
    return 0;
}
