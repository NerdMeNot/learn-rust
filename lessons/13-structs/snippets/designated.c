#include <stdio.h>

struct User {
    const char *name;
    unsigned age;
    int active;
};

int main(void) {
    // Designators in any order; `active` is left out and becomes 0.
    struct User u = {.age = 30, .name = "ana"};
    printf("%s %u %d\n", u.name, u.age, u.active);
    return 0;
}
