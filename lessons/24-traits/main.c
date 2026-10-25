#include <ctype.h>
#include <stdio.h>

// C has no traits: a "Speaker" is a struct of function pointers
struct Speaker {
    void (*speak)(const void *self, char *out, size_t n);
};

struct Dog { int unused; };
struct Robot { unsigned id; };

void dog_speak(const void *self, char *out, size_t n) {
    (void)self;
    snprintf(out, n, "woof");
}

void robot_speak(const void *self, char *out, size_t n) {
    const struct Robot *r = self;
    snprintf(out, n, "beep %u", r->id);
}

const struct Speaker DOG_SPEAKER = {dog_speak};
const struct Speaker ROBOT_SPEAKER = {robot_speak};

// the "default method": works for any Speaker
void shout(const struct Speaker *sp, const void *self, char *out, size_t n) {
    sp->speak(self, out, n);
    for (char *p = out; *p; p++) *p = (char)toupper((unsigned char)*p);
}

void introduce(const struct Speaker *sp, const void *self) {
    char a[64], b[64];
    sp->speak(self, a, sizeof a);
    shout(sp, self, b, sizeof b);
    printf("%s / %s\n", a, b);
}

int main(void) {
    struct Dog dog = {0};
    struct Robot robot = {7};
    introduce(&DOG_SPEAKER, &dog);
    introduce(&ROBOT_SPEAKER, &robot);
    return 0;
}
