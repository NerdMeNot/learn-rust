#include <pthread.h>
#include <stdio.h>

void *worker(void *arg) {
    const char **names = arg;   // points at main's local array
    size_t len = 0;
    while (names[len] != NULL) len++;
    printf("%zu\n", len);
    return NULL;
}

int main(void) {
    const char *names[] = {"a", "b", "c", NULL};
    pthread_t t;
    pthread_create(&t, NULL, worker, names);
    pthread_join(t, NULL);
    return 0;
}
