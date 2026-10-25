#include <stdio.h>
#include <stddef.h>
#include "bridge.h"

int main(void) {
    printf("C, Reading: size %zu, id at %zu, value at %zu, flag at %zu\n",
           sizeof(Reading), offsetof(Reading, id), offsetof(Reading, value),
           offsetof(Reading, flag));
    return 0;
}
