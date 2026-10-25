#include <stdio.h>

static int log_msg(const char *msg) {
    printf("log: %s\n", msg);
    return 1;
}

int main(void) {
    int c = (log_msg("first"), 2);
    printf("comma: %d\n", c);
    int area = ({ int w = 3; int h = 4; w * h; });
    printf("statement expression: %d\n", area);
    return 0;
}
