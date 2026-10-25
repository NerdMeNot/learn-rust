#include <stdio.h>

struct Person {
    int age;
};

static void birthday_value(struct Person p) {
    p.age++;
}

static void birthday_pointer(struct Person *p) {
    p->age++;
}

int main(void) {
    struct Person people[2] = {{36}, {54}};

    for (int i = 0; i < 2; i++) {
        struct Person p = people[i]; /* a copy of the struct */
        p.age++;
    }
    printf("after copying each into p: %d, %d\n", people[0].age, people[1].age);

    for (int i = 0; i < 2; i++) {
        struct Person *p = &people[i]; /* the element itself */
        p->age++;
    }
    printf("after p = &people[i]:      %d, %d\n", people[0].age, people[1].age);

    birthday_value(people[0]);
    printf("after birthday_value:      %d\n", people[0].age);
    birthday_pointer(&people[0]);
    printf("after birthday_pointer:    %d\n", people[0].age);
    return 0;
}
