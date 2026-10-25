#include <stdio.h>

int main(void) {
    int day = 6;
    const char *kind = "";
    switch (day) {
    case 1: case 2: case 3: case 4: case 5:
        kind = "weekday";
        break;
    case 6: case 7:
        kind = "weekend";
        break;
    }
    printf("%s\n", kind);
    return 0;
}
