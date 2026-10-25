#include <stdio.h>

int main(void) {
    const char *names[] = {"ana", "bo"};
    const char *first = names[0];   // copies the pointer, not the text
    printf("%s of %d\n", first, (int)(sizeof names / sizeof names[0]));
    return 0;
}
