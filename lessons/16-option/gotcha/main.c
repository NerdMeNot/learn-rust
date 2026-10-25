#include <stdio.h>

struct User {
    const char *name;
};

static struct User ana = {"ana"};

struct User *find_user(unsigned id) {
    if (id == 1) return &ana;
    return NULL;
}

int main(void) {
    struct User *u = find_user(2);
    printf("%s\n", u->name);
    return 0;
}
