#include <stdio.h>
#include <string.h>

// A pointer and a length, like &str. Nothing in the types says the result of
// longest() points into a or b, or for how long that stays valid: that is a comment's job.
struct View {
    const char *ptr;
    size_t len;
};

static struct View longest(struct View a, struct View b) {
    return a.len >= b.len ? a : b;
}

struct Excerpt {
    struct View part;
};

int main(void) {
    const char *novel = "Call me Ishmael. Some years ago...";
    struct View first = {novel, (size_t)(strchr(novel, '.') - novel)};
    struct Excerpt excerpt = {first};
    struct View other = {"short", 5};
    struct View l = longest(excerpt.part, other);
    printf("longest: %.*s\n", (int)l.len, l.ptr);
    printf("excerpt length: %zu\n", excerpt.part.len);
    return 0;
}
