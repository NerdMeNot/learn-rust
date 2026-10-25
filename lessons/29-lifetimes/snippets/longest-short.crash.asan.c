#include <stdio.h>
#include <string.h>

static const char *longest(const char *a, const char *b) {
    return strlen(a) >= strlen(b) ? a : b;
}

int main(void) {
    char s1[] = "short";
    const char *result;
    {
        char s2[] = "a long second string";
        result = longest(s1, s2);
    }
    printf("longest: %s\n", result);
    return 0;
}
