#include <stdio.h>

int main(void) {
    const char *names[] = {"ana", "bo"};
    size_t len = sizeof names / sizeof names[0];
    for (size_t i = 0; i < len; i++) {
        printf("hi %s\n", names[i]);
    }
    printf("%zu names\n", len);
    return 0;
}
