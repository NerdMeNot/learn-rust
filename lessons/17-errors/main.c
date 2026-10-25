#include <ctype.h>
#include <stdio.h>

// C has no Result: return an error message (NULL means success)
// and write the value through a pointer
const char *parse_age(const char *s, unsigned *out) {
    if (*s == '\0') return "cannot parse integer from empty string";
    unsigned n = 0;
    for (; *s; s++) {
        if (!isdigit((unsigned char)*s)) return "invalid digit found in string";
        n = n * 10 + (unsigned)(*s - '0');
    }
    *out = n;
    return NULL;
}

int main(void) {
    const char *inputs[] = {"42", "abc"};
    for (int i = 0; i < 2; i++) {
        unsigned n;
        const char *err = parse_age(inputs[i], &n);
        if (err == NULL) {
            printf("age %u\n", n);
        } else {
            printf("error: %s\n", err);
        }
    }
    return 0;
}
