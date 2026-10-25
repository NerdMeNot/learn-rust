#include <stdio.h>

struct Config {
    const char *name;
    unsigned short port;
    int verbose;
};

int main(void) {
    struct Config base = {"service", 8080, 0};
    struct Config test = base; /* copies the fields, including the name pointer */
    test.port = 9000;
    printf("%s %d %d\n", base.name, base.port, base.verbose);
    printf("%s %d %d\n", test.name, test.port, test.verbose);
    return 0;
}
