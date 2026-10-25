#include <stdio.h>
#include "bridge.h"

int main(void) {
    char *text = greeting("leak");
    printf("%s\n", text);
    /* No greeting_free(text): Rust allocated this string, and nobody tells Rust to free it. */
    return 0;
}
