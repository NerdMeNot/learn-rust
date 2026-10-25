#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// three functions, one return type, three different promises
char *into_input(char *text) { return strchr(text, ' '); }             // points into `text`
char *static_buffer(void) { static char buf[] = "static"; return buf; } // lives for the whole run
char *new_memory(void) { return strdup("malloc"); }                     // the caller must free it

int main(void) {
    char text[] = "hello world";
    char *a = into_input(text);
    char *b = static_buffer();
    char *c = new_memory();
    printf("into_input:    %s (%ld bytes into text)\n", a, (long)(a - text));
    printf("static_buffer: %s\n", b);
    printf("new_memory:    %s\n", c);
    free(c); // free(a) or free(b) would be a bug, and the types would not warn you
    return 0;
}
