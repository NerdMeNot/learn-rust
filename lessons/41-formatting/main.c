#include <stdio.h>

struct Money {
    long cents;
};

// C has no Display: write the text into a buffer the caller gives you
void money_str(char *buf, size_t size, struct Money m) {
    snprintf(buf, size, "$%ld.%02ld", m.cents / 100, m.cents % 100);
}

struct Item {
    const char *name;
    unsigned qty;
};

int main(void) {
    int n = 42;
    printf("[%6d] [%-6d] [%06d]\n", n, n, n); // no centering in printf
    // no %b for binary before C23
    printf("%.2f %e %#x\n", 3.14159, 1234.5, n);

    struct Money price = {1999};
    char money[32];
    money_str(money, sizeof money, price);
    printf("total: %s\n", money);

    // snprintf builds a string, and returns how long it is
    char label[64];
    int len = snprintf(label, sizeof label, "%s each", money);
    printf("%s (%d chars)\n", label, len);

    // no Debug either: print each field by hand
    struct Item item = {"pen", 3};
    printf("%u x %s\n", item.qty, item.name);
    printf("Item { name: \"%s\", qty: %u }\n", item.name, item.qty);
    return 0;
}
