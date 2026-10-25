#include <stdio.h>
#include <string.h>

int main(void) {
    const char *line = "mode";
    const char *eq = strchr(line, '=');     /* NULL: there is no '=' */
    printf("value: %s\n", eq + 1);
    return 0;
}
