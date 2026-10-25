#include <stdio.h>

struct Speaker; // the "interface": a table of function pointers, filled in by hand

struct SpeakerVTable {
    const char* (*speak)(const struct Speaker*);
};

struct Speaker {
    const struct SpeakerVTable* vtable;
};

struct Dog {
    struct Speaker base; // must come first, so a Dog* can be used as a Speaker*
};

static const char* dog_speak(const struct Speaker* self) {
    (void)self;
    return "woof";
}

static const struct SpeakerVTable DOG_VTABLE = {dog_speak};

static void introduce(const struct Speaker* s) {
    printf("%s\n", s->vtable->speak(s));
}

int main(void) {
    struct Dog dog = {{&DOG_VTABLE}};
    introduce(&dog.base);
    return 0;
}
