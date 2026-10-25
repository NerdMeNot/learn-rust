#include <stdio.h>

int main(void) {
    int ages[3] = {30, 40, 50};
    for (int i = 0; i < 3; i++) {
        int age = ages[i];
        age += 1;
    }
    printf("[%d, %d, %d]\n", ages[0], ages[1], ages[2]);
    return 0;
}
