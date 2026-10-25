#include <stdio.h>

enum ShapeKind { CIRCLE, RECT };

struct Shape {
    enum ShapeKind kind;
    union {
        double r;
        struct { double w, h; } rect;
    };
};

int main(void) {
    printf("sizeof(struct Shape): %zu\n", sizeof(struct Shape));
    return 0;
}
