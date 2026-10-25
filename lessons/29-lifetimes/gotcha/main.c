#include <stdio.h>
#include <string.h>

static const char *longest(const char *a, const char *b) {
    return strlen(a) >= strlen(b) ? a : b;
}

int main(void) {
    char s1[] = "a long first string";
    const char *result;
    {
        char s2[] = "short";
        result = longest(s1, s2);
    }
    printf("longest: %s\n", result);
    return 0;
}
