#include <stdio.h>
#include <stdlib.h>

// A global that any function may change.
int visits = 0;

static void visit(void) { visits++; }

static void check(int ok, const char *what) {
    if (!ok) {
        fprintf(stderr, "failed: %s\n", what);
        exit(1);
    }
}

int main(void) {
    // ++ and --, also inside expressions, and --x decrements
    int a[] = {10, 20, 30};
    int i = 0;
    int first = a[i++];
    int second = a[++i];
    check(first == 10 && second == 30 && i == 2, "inc");
    const int *p = a;
    int via = *p++;
    check(via == 10 && *p == 20, "*p++");
    int j = 5;
    int pre = --j;
    check(pre == 4 && j == 4, "--x decrements");

    // the ternary operator
    const char *label = i > 1 ? "big" : "small";
    check(label[0] == 'b', "ternary");

    // do-while runs the body once even when the test is false
    int runs = 0;
    do {
        runs++;
    } while (0);
    check(runs == 1, "do-while");

    // for (;;) and while (1)
    int spins = 0;
    for (;;) {
        spins++;
        break;
    }
    while (1) {
        spins++;
        break;
    }
    check(spins == 2, "forever");

    // NULL, and a pointer used as a condition
    const int *np = NULL;
    check(np == NULL && !np, "null");

    // implicit conversions: a double to an int, an int to a long, an int to a double
    int n = 3.9;
    long l = i;
    double d = 3;
    check(n == 3 && l == 2 && d == 3.0, "convert");

    // a cast, and character arithmetic
    check((double)n == 3.0 && '7' - '0' == 7, "cast");

    // a number and a string used as a condition
    int count = 4;
    int hit = 0;
    if (count) {
        hit = 1;
    }
    const char *s = "ab";
    int chars = 0;
    while (*s++) {
        chars++;
    }
    check(hit == 1 && chars == 2, "truthy");

    // -> on a pointer
    struct P {
        int x;
    } pt = {7};
    struct P *pp = &pt;
    check(pp->x == 7, "arrow");

    // switch falls through
    int out = 0;
    switch (1) {
    case 1:
        out += 1;
    case 2:
        out += 10;
        break;
    default:
        out += 100;
    }
    check(out == 11, "fallthrough");

    // goto
    int k = 0;
again:
    k++;
    if (k < 3) {
        goto again;
    }
    check(k == 3, "goto");

    visit();
    visit();
    check(visits == 2, "global");

    printf("c: all checks passed\n");
    return 0;
}
