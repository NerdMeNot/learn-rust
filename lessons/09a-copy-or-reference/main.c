#include <stdio.h>

int main(void) {
    int ages[3] = {30, 40, 50};

    // A copy: `age` is a new variable holding the element's value.
    for (int i = 0; i < 3; i++) {
        int age = ages[i];
        age++;
        printf("the copy is now %d\n", age);
    }
    printf("ages after the copy loop:     [%d, %d, %d]\n", ages[0], ages[1], ages[2]);

    // The elements themselves: a pointer to each element.
    for (int i = 0; i < 3; i++) {
        int *age = &ages[i];
        (*age)++;
    }
    printf("ages after the in-place loop: [%d, %d, %d]\n", ages[0], ages[1], ages[2]);
    return 0;
}
