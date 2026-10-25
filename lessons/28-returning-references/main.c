#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// returns NEW memory: the caller owns it and must free it
char *make_greeting(const char *name) {
    size_t n = strlen("hello ") + strlen(name) + 1;
    char *s = malloc(n);
    snprintf(s, n, "hello %s", name);
    return s;
}

// returns a pointer into the input: the end of the first word, no copy
const char *first_word_end(const char *text) {
    const char *space = strchr(text, ' ');
    return space ? space : text + strlen(text);
}

int main(void) {
    char *g = make_greeting("rust");
    const char *end = first_word_end(g);
    printf("%s / %.*s\n", g, (int)(end - g), g);
    free(g);
    return 0;
}
