#include <errno.h>
#include <stdio.h>
#include <stdlib.h>

// strtol reports trouble three ways: errno, the end pointer, and the value.
// A caller has to check all of them, and nothing forces it to.
int parse(const char *s, long *out) {
    char *end;
    errno = 0;
    long n = strtol(s, &end, 10);
    if (errno == ERANGE) return -1;      // too big
    if (end == s || *end != '\0') return -2;  // no digits, or junk after them
    *out = n;
    return 0;
}

int main(void) {
    const char *inputs[] = {"42", "abc", "", "99999999999999999999"};
    for (int i = 0; i < 4; i++) {
        long n = 0;
        int code = parse(inputs[i], &n);
        printf("parse(\"%s\"): code %d, value %ld\n", inputs[i], code, n);
    }
    return 0;
}
