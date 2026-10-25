#include <stdio.h>
#include "bridge.h"

int main(void) {
    char *text = greeting("twice");
    printf("%s\n", text);
    greeting_free(text);
    greeting_free(text); /* the second call hands Rust a pointer that is already freed */
    return 0;
}
