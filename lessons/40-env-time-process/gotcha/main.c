#include <stdio.h>
#include <stdlib.h>

int main(void) {
    const char *port = getenv("RBC_PORT");
    printf("listening on port %s\n", port);
    return 0;
}
