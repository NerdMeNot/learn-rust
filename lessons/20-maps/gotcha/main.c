#include <stdio.h>
#include <string.h>

struct Entry {
    const char *key;
    unsigned value;
};

// returns a pointer to the value, or NULL when the key is missing
unsigned *get(struct Entry *items, int len, const char *key) {
    for (int i = 0; i < len; i++) {
        if (strcmp(items[i].key, key) == 0) return &items[i].value;
    }
    return NULL;
}

int main(void) {
    struct Entry ages[] = {{"ana", 30}};
    unsigned age = *get(ages, 1, "ana");
    printf("%u\n", age);
    return 0;
}
