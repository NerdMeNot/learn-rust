#include <stdio.h>

struct Item {
    const char *name;
    int count;
};

struct Inventory {
    struct Item items[3];
    const char *log[3];
    int log_len;
};

static void note(struct Inventory *inv, const char *name) {
    inv->log[inv->log_len++] = name;
}

static void check(struct Inventory *inv) {
    for (int i = 0; i < 3; i++) {
        if (inv->items[i].count == 0) {
            note(inv, inv->items[i].name);
        }
    }
}

int main(void) {
    struct Inventory inv = {{{"bolt", 0}, {"nut", 12}, {"washer", 0}}, {0}, 0};
    check(&inv);
    printf("log: [");
    for (int i = 0; i < inv.log_len; i++) {
        printf("%s\"out of stock: %s\"", i ? ", " : "", inv.log[i]);
    }
    printf("]\n");
}
