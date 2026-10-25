#include <stddef.h>
#include <stdint.h>
#include <stdio.h>

struct S {
    uint8_t a;
    uint32_t b;
    uint8_t c;
};

int main(void) {
    printf("S: size %zu, align %zu\n", sizeof(struct S), _Alignof(struct S));
    printf("  offsets: a %zu, b %zu, c %zu\n", offsetof(struct S, a),
           offsetof(struct S, b), offsetof(struct S, c));
    return 0;
}
