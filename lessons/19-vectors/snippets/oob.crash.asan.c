#include <stdio.h>
#include <stdlib.h>

int main(void) {
    int *v = malloc(3 * sizeof(int));
    v[0] = 10; v[1] = 20; v[2] = 30;
    int i = 10;
    printf("v[%d] is %d\n", i, v[i]);   // no check: reads past the end
    free(v);
    return 0;
}
