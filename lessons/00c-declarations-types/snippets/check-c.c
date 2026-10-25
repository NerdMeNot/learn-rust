#include <assert.h>
#include <stdint.h>
#include <stdio.h>

#define MAX 100

static int add(int a, int b) { return a + b; }

int main(void) {
    int x = 5;
    const int y = 5;
    int a[3] = {1, 2, 3};
    assert(x == y && MAX == 100 && add(1, 2) == 3 && a[2] == 3);

    double price = 2.9;
    assert((int)price == 2 && (int)-price == -2 && (double)x == 5.0);

    assert(sizeof('a') == sizeof(int)); /* in C a character constant is an int */
    assert(0377 == 255 && 017 == 15);   /* a leading zero means octal */
    assert(0xff == 255 && 0b101 == 5);  /* 0b literals: GCC extension, C23 */
    assert(5u == 5 && 5UL == 5 && 2.5f == 2.5);
    return 0;
}
