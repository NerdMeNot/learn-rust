#include <stdio.h>

enum Kind { INT, FLOAT };

struct Value {
    enum Kind kind;
    union {
        int i;
        float f;
    };
};

void show(const struct Value *v) {
    switch (v->kind) {
    case INT:   printf("int %d\n", v->i); break;
    case FLOAT: printf("float %g\n", v->f); break;
    }
}

int main(void) {
    struct Value v = {.kind = FLOAT, .f = 1.0f};
    show(&v);
    v.kind = INT; // the tag changed, the bytes did not
    show(&v);
    return 0;
}
