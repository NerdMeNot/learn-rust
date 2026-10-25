#include <stdio.h>

int main(int argc, char **argv) {
    const char *text = argc > 5 ? argv[1] : "$19.99 and more";
    char buf[8];
    int wanted = snprintf(buf, sizeof buf, "%s each", text);
    printf("returned %d, kept \"%s\"\n", wanted, buf);
    return 0;
}
