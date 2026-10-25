#include <stdio.h>
#include "bridge.h"

int main(void) {
    int out = 0;
    int status = checked_div(7, 2, &out);
    printf("checked_div(7, 2): status %d, out %d\n", status, out);
    status = checked_div(1, 0, &out);
    printf("checked_div(1, 0): status %d\n", status);
    return 0;
}
