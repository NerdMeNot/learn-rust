#include <stdio.h>

char *make_greeting(const char *name) {
    char s[32];
    snprintf(s, sizeof s, "hello %s", name);
    return s;
}

int main(void) {
    printf("%s\n", make_greeting("rust"));
    return 0;
}
