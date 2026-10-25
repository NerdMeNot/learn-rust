#include <pthread.h>
#include <stdio.h>
#include <string.h>

static const char *words[2] = {"alpha", "beta"};

void *echo(void *arg) {
    return arg;                       // a thread hands back a pointer
}

void *length(void *arg) {
    return (void *)strlen(*(const char **)arg);
}

int main(void) {
    pthread_t t;
    void *back;
    pthread_create(&t, NULL, echo, words);
    pthread_join(t, &back);           // wait, and collect the result
    const char **w = back;
    printf("moved there and back: [\"%s\", \"%s\"]\n", w[0], w[1]);

    // two threads read the same array at once; nothing checks that this is safe
    pthread_t a, b;
    void *la, *lb;
    pthread_create(&a, NULL, length, &words[0]);
    pthread_create(&b, NULL, length, &words[1]);
    pthread_join(a, &la);
    pthread_join(b, &lb);
    printf("lengths read in parallel: %zu and %zu\n", (size_t)la, (size_t)lb);
    return 0;
}
