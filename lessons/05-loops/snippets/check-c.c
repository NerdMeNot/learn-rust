#include <assert.h>
#include <stdio.h>

int main(void) {
    int n = 0;
    do {
        n += 5;
    } while (n < 3);
    assert(n == 5); /* a do-while runs the body once before testing */

    int count = 0;
    for (;;) {
        count++;
        if (count == 3) {
            break;
        }
    }
    assert(count == 3);

    /* the bound in i < n is read again on every pass */
    int n2 = 3, runs = 0;
    for (int i = 0; i < n2; i++) {
        if (i == 0) {
            n2 = 5;
        }
        runs++;
    }
    assert(runs == 5);

    /* continue in a for goes on to the step */
    int steps = 0;
    for (int i = 0; i < 3; i++) {
        if (i == 1) {
            continue;
        }
        steps++;
    }
    assert(steps == 2);

    /* continue in a do-while goes on to the test, which ends the loop */
    int k = 0, passes = 0;
    do {
        k++;
        passes++;
        if (k >= 3) {
            continue;
        }
    } while (k < 3);
    assert(k == 3 && passes == 3);
    puts("ok");
    return 0;
}
