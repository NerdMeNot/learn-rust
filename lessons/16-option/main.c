#include <stdio.h>

struct User {
    const char *name;
};

static struct User ana = {"ana"};

// C has no Option: return a pointer, and NULL means "none"
const struct User *find_user(unsigned id) {
    if (id == 1) {
        return &ana;
    }
    return NULL;
}

int main(void) {
    const struct User *u = find_user(1);
    if (u != NULL) {
        printf("found %s\n", u->name);
    } else {
        printf("no user\n");
    }

    u = find_user(2);
    if (u != NULL) {
        printf("found %s\n", u->name);
    } else {
        printf("no user\n");
    }
    return 0;
}
