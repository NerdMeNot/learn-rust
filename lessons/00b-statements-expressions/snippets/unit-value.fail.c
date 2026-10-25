#include <stdio.h>

void log_msg(const char *msg) {
    printf("log: %s\n", msg);
}

int main(void) {
    int r = log_msg("hello");
    printf("r: %d\n", r);
    return 0;
}
