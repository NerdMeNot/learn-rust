#include <stdbool.h>
#include <stdio.h>

struct User {
    const char *name;
    unsigned age;
    bool active;
};

int main(void) {
    struct User u = {
        .name = "ana",
        .age = 30,
        .active = true,
    };
    printf("%s is %u, active=%s\n", u.name, u.age, u.active ? "true" : "false");
    // C has no {:?}: print each field yourself
    printf("User { name: \"%s\", age: %u, active: %s }\n",
           u.name, u.age, u.active ? "true" : "false");
    return 0;
}
