#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

int main(void) {
    assert(atoi("4x") == 4);
    assert(atoi("x") == 0);
    assert(atoi("42") == 42);
    printf("all checks passed\n");
    return 0;
}
