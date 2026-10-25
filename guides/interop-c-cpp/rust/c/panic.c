#include <stdio.h>
#include "bridge.h"

int main(void) {
    printf("explode(0) = %d\n", explode(0));
    fflush(stdout);
    printf("explode(1) = %d\n", explode(1));
    printf("not reached\n");
    return 0;
}
