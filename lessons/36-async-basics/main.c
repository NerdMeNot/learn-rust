#include <stdio.h>

static void task(void) { puts("task body runs"); }

int main(void) {
    puts("start");
    /* C has no futures or async/await. A function pointer is the nearest thing:
       holding it runs nothing until it is called. */
    void (*created)(void) = task;
    puts("task created");
    created();
    puts("end");
    return 0;
}
