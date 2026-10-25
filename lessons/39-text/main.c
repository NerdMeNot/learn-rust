#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    const char *line = "  name=Ana, age=30, city=Lisbon  ";

    // trim: move the start forward and the end back. Nothing is copied.
    const char *start = line;
    while (isspace((unsigned char)*start)) start++;
    const char *end = start + strlen(start);
    while (end > start && isspace((unsigned char)end[-1])) end--;
    int clean_len = (int)(end - start);
    printf("[%.*s]\n", clean_len, start);

    // split on ',', skip spaces, split once on '='
    const char *p = start;
    while (p < end) {
        const char *comma = memchr(p, ',', (size_t)(end - p));
        const char *stop = comma ? comma : end;
        while (p < stop && *p == ' ') p++;
        const char *eq = memchr(p, '=', (size_t)(stop - p));
        if (eq) {
            printf("%5.*s -> %.*s\n", (int)(eq - p), p, (int)(stop - eq - 1), eq + 1);
        }
        p = stop + 1;
    }

    // text to number: strtol tells you where it stopped
    char *rest;
    long age = strtol("30", &rest, 10);
    if (*rest != '\0') age = 0;
    printf("next year: %ld\n", age + 1);

    // upper-case and replace ", " with " | " into a new buffer
    char shout[64];
    int n = 0;
    for (const char *c = start; c < end; c++) {
        if (c[0] == ',' && c[1] == ' ') {
            n += snprintf(shout + n, sizeof shout - (size_t)n, " | ");
            c++;
        } else {
            shout[n++] = (char)toupper((unsigned char)*c);
        }
    }
    shout[n] = '\0';
    printf("%s\n", shout);

    printf("starts with name: %s\n", strncmp(start, "name", 4) == 0 ? "true" : "false");
    const char *found = strstr(start, "age"); // NULL when not found
    printf("position of age: %d\n", found ? (int)(found - start) : -1);

    const char *words[] = {"build", "with", "rust"};
    char joined[32] = "";
    for (int i = 0; i < 3; i++) {
        if (i > 0) strcat(joined, "-");
        strcat(joined, words[i]);
    }
    printf("%s\n", joined);

    // grow a string: track the length yourself
    char out[32];
    int len = 0;
    for (int i = 1; i <= 3; i++) {
        len += snprintf(out + len, sizeof out - (size_t)len, "%d;", i);
    }
    printf("%s\n", out);
    return 0;
}
