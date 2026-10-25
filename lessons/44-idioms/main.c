#include <stdio.h>
#include <stdlib.h>

/* The struct is a plain value: nothing stops `struct Port p = {0};`.
   In a library, the header would only declare `struct Port;` so callers
   could hold a pointer but not build one. */
struct Port {
    unsigned short number;
};

/* Returns 0 on success; on failure returns -1 and sets *error. */
static int parse_port(const char *text, struct Port *out, const char **error) {
    char *end;
    unsigned long n = strtoul(text, &end, 10);
    if (*text == '\0' || *end != '\0') {
        *error = "not a number";
        return -1;
    }
    if (n < 1 || n > 65535) {
        static char message[64];
        snprintf(message, sizeof message, "%lu is outside 1-65535", n);
        *error = message;
        return -1;
    }
    out->number = (unsigned short)n;
    return 0;
}

static void listen_on(const struct Port *port) {
    printf("listening on %d\n", port->number);
}

int main(void) {
    const char *inputs[] = {"8080", "0", "http"};
    for (int i = 0; i < 3; i++) {
        struct Port port;
        const char *error;
        if (parse_port(inputs[i], &port, &error) == 0) {
            listen_on(&port);
        } else {
            printf("\"%s\": %s\n", inputs[i], error);
        }
    }
    return 0;
}
