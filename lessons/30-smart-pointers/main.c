#include <stdio.h>
#include <stdlib.h>

struct Log {
    const char *lines[8];
    int count;
};

int main(void) {
    // two pointers to one struct; nothing counts the owners
    struct Log *log = calloc(1, sizeof *log);
    struct Log *other = log;

    log->lines[log->count++] = "from log";
    other->lines[other->count++] = "from other";

    printf("log: ");
    for (int i = 0; i < log->count; i++) {
        printf("%s%s", i ? ", " : "", log->lines[i]);
    }
    printf("\n");
    printf("same list: %s\n", log == other ? "true" : "false");

    free(log); // exactly once, and only because we remembered
    return 0;
}
