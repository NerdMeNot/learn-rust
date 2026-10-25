#include <stdio.h>

static int log_msg(const char *msg) {
    printf("log: %s\n", msg);
    return 1;
}

int main(void) {
    int n = 7;

    const char *size = n > 5 ? "big" : "small";
    printf("ternary: %s\n", size);

    int area = ({ int w = 3; int h = 4; w * h; });
    printf("block: %d\n", area);

    const char *name;
    switch (2) {
    case 1:
        name = "one";
        break;
    case 2:
        name = "two";
        break;
    default:
        name = "many";
    }
    printf("switch: %s\n", name);

    int a = 1, b = 2;
    int tmp = a;
    a = b;
    b = tmp;
    printf("swap: %d %d\n", a, b);

    int c = (log_msg("first"), 2);
    printf("comma: %d\n", c);
    return 0;
}
