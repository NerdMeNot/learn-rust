#define NDEBUG
#include <assert.h>
#include <stdio.h>

int main(void) {
    assert(1 + 1 == 3);
    printf("assert did not stop the program\n");
    return 0;
}
