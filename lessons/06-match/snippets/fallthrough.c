#include <stdio.h>

int main(void) {
    int n = 1;
    int total = 0;
    switch (n) {
    case 1:
        total += 1;
    case 2:
        total += 10;
        break;
    }
    printf("%d\n", total);
    return 0;
}
