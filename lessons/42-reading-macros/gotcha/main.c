#include <stdio.h>

struct Entry {
    const char *name;
    int age;
};

int main(void) {
    struct Entry ages[] = {{"Ada", 36}, {"Linus", 28}};
    printf("entries: %zu\n", sizeof ages / sizeof ages[0]);
    return 0;
}
