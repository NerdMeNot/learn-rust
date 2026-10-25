#include <stdio.h>

enum Light { RED, YELLOW, GREEN };

int main(void) {
    enum Light light = YELLOW;
    const char *action = "no action";
    switch (light) {
    case RED:
        action = "stop";
        break;
    case GREEN:
        action = "go";
        break;
    }
    printf("%s\n", action);
    return 0;
}
