#include <stdio.h>

// C enums hold no data, so pair a tag with a union (a "tagged union")
enum ShapeKind { CIRCLE, RECT };

struct Shape {
    enum ShapeKind kind;
    union {
        double r;                       // CIRCLE: radius
        struct { double w, h; } rect;   // RECT: width, height
    };
};

double area(const struct Shape *s) {
    switch (s->kind) {
    case CIRCLE: return 3.14 * s->r * s->r;
    case RECT:   return s->rect.w * s->rect.h;
    }
    return 0;
}

int main(void) {
    struct Shape shapes[] = {
        {.kind = CIRCLE, .r = 1.0},
        {.kind = RECT, .rect = {2.0, 3.0}},
    };
    for (int i = 0; i < 2; i++) {
        printf("%g\n", area(&shapes[i]));
    }
    return 0;
}
