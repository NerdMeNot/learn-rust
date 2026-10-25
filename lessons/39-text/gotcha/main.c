#include <stdio.h>
#include <string.h>

int main(void) {
    char *line = "ana,30,lisbon";
    char *name = strtok(line, ",");
    printf("%s\n", name);
    return 0;
}
