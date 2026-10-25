#include <stdio.h>

/* 1. typedef makes a second name, not a second type. */
typedef int UserId;
typedef int OrderId;

static void load_user(UserId id) {
    printf("user #%d\n", id);
}

/* 2. Designated initializers: fields you leave out are zero. */
struct Config {
    const char *host;
    int port;
    int retries;
};

int main(void) {
    OrderId order = 7;
    load_user(order); /* compiles with -Wall -Wextra */

    struct Config c = {.host = "localhost", .port = 9000};
    printf("host=%s port=%d retries=%d\n", c.host, c.port, c.retries);
    return 0;
}
