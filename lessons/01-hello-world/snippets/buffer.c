#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* The program under test prints a line, then dies without a clean exit. */
int main(int argc, char **argv) {
    if (argc > 1 && strcmp(argv[1], "child") == 0) {
        printf("a line, with a newline\n");
        abort();
    }
    FILE *p = popen("./main child 2>/dev/null", "r");
    char buf[100] = "";
    size_t n = fread(buf, 1, sizeof buf - 1, p);
    buf[n] = '\0';
    pclose(p);
    printf("stdout is a pipe: read %zu bytes: \"%s\"\n", n, buf);
    return 0;
}
