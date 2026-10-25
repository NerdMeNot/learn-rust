#include <stdio.h>
#include <string.h>

int main(void) {
    const char *line = "mode";
    const char *eq = strchr(line, '=');
    printf("%.*s -> %s\n", (int)(eq - line), line, eq + 1);
    return 0;
}
