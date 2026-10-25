#include <stdio.h>

int main(void) {
    const char *cmd = "go";
    switch (cmd) {
    case "go":
        printf("moving\n");
        break;
    default:
        printf("unknown command\n");
    }
    return 0;
}
