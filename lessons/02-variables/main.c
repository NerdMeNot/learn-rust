#include <stdio.h>

int main(void) {
    const char *const name = "gopher";  // neither the pointer nor the text can change
    int count = 1;                // C variables can change by default
    count += 1;

    count = count * 10;           // C has no shadowing in the same block: reuse it
    printf("%s %d\n", name, count);
    return 0;
}
