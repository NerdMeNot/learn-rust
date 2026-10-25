#include <stdio.h>

enum ShapeKind { CIRCLE, RECT, TRIANGLE };   // TRIANGLE is new!

struct Shape {
    enum ShapeKind kind;
    double a, b;
};

double area(struct Shape s) {
    switch (s.kind) {
    case CIRCLE: return 3.14 * s.a * s.a;
    case RECT:   return s.a * s.b;
    }
    return 0;
}

int main(void) {
    struct Shape t = {TRIANGLE, 2.0, 3.0};
    printf("%g\n", area(t));
    return 0;
}
