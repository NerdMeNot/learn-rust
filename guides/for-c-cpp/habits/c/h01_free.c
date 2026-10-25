#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    char *a = malloc(6);
    strcpy(a, "hello");
    char *b = a;        /* copies the pointer, not the text */
    printf("%s\n", b);
    free(a);
    free(b);            /* the same block again */
    return 0;
}
