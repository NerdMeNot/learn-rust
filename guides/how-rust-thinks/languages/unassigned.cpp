#include <stdio.h>

int main(void) {
    int ready = 1;
    int port;
    if (ready) {
        port = 8080;
    }
    printf("listening on %d\n", port);
    return 0;
}
