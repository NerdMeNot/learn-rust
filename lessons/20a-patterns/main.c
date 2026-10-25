#include <stdio.h>
#include <string.h>

struct Point {
    int x, y;
};

int main(void) {
    // no destructuring: read the fields
    struct Point p = {3, -2};
    int x = p.x, y = p.y;
    printf("x = %d, y = %d\n", x, y);

    const char *abc = "abc";
    for (int i = 0; abc[i] != '\0'; i++) {
        printf("%d: %c\n", i, abc[i]);
    }

    // strchr returns a pointer to the '=' or NULL
    const char *line = "mode=fast";
    const char *eq = strchr(line, '=');
    if (eq != NULL) {
        printf("%.*s -> %s\n", (int)(eq - line), line, eq + 1);
    }

    int a[] = {10, 20, 30, 40};
    printf("first = %d, last = %d\n", a[0], a[3]);
    return 0;
}
