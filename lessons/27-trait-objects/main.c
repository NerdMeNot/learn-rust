#include <stdio.h>

// C has no traits. A "dyn Shape" is two pointers you carry yourself:
// one to the data and one to a table of functions.
struct ShapeVTable {
    const char *(*name)(const void *self);
    unsigned (*area)(const void *self);
};

struct Shape {
    const void *data;
    const struct ShapeVTable *vtable;
};

struct Square { unsigned side; };
struct Rect { unsigned w, h; };

const char *square_name(const void *self) { (void)self; return "square"; }
unsigned square_area(const void *self) {
    const struct Square *s = self;
    return s->side * s->side;
}

const char *rect_name(const void *self) { (void)self; return "rect"; }
unsigned rect_area(const void *self) {
    const struct Rect *r = self;
    return r->w * r->h;
}

const struct ShapeVTable SQUARE_VTABLE = {square_name, square_area};
const struct ShapeVTable RECT_VTABLE = {rect_name, rect_area};

int main(void) {
    struct Square sq = {3};
    struct Rect rect = {4, 6};
    struct Shape shapes[] = {{&sq, &SQUARE_VTABLE}, {&rect, &RECT_VTABLE}};

    unsigned total = 0;
    for (int i = 0; i < 2; i++) {
        struct Shape s = shapes[i];
        printf("%s: %u\n", s.vtable->name(s.data), s.vtable->area(s.data));
        total += s.vtable->area(s.data);
    }
    printf("total: %u\n", total);
    return 0;
}
