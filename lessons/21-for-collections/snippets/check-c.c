// Backs the C cells of the tables in lesson.mdx. Never shown.
#include <assert.h>
#include <stddef.h>
#include <stdio.h>

int main(void) {
    int a[] = {1, 2, 3};
    int sum = 0;
    for (int *p = a; p < a + 3; p++) {
        sum += *p;
    }
    assert(sum == 6);
    int back = 0;
    for (size_t i = 3; i-- > 0;) {
        back = back * 10 + a[i];
    }
    assert(back == 321);
    puts("ok");
    return 0;
}
