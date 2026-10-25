#include <stdbool.h>
#include <stdio.h>

struct User {
    const char *name;
    unsigned age;
    bool active;
};

int main(void) {
    struct User u = {.name = "ana"};
    printf("%s %u %d\n", u.name, u.age, u.active);
    return 0;
}
