#include <stdio.h>

int next(void) {
    static int calls = 0; // one variable that lives for the whole program
    calls += 1;
    return calls;
}

int main(void) {
    printf("%d %d %d\n", next(), next(), next());
    return 0;
}
