#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

// the list and its lock live together; every access takes the lock
struct Shared {
    pthread_mutex_t lock;
    int items[3];
    int len;
};

struct Arg {
    struct Shared *shared;
    int i;
};

void *worker(void *p) {
    struct Arg *a = p;
    pthread_mutex_lock(&a->shared->lock);        // lock, push, unlock
    a->shared->items[a->shared->len++] = a->i * 10;
    pthread_mutex_unlock(&a->shared->lock);
    return NULL;
}

int cmp_int(const void *x, const void *y) {
    return *(const int *)x - *(const int *)y;
}

int main(void) {
    struct Shared results = {.lock = PTHREAD_MUTEX_INITIALIZER};
    pthread_t threads[3];
    struct Arg args[3];

    for (int i = 0; i < 3; i++) {
        args[i] = (struct Arg){&results, i};
        pthread_create(&threads[i], NULL, worker, &args[i]);
    }
    for (int i = 0; i < 3; i++) {
        pthread_join(threads[i], NULL);
    }

    pthread_mutex_lock(&results.lock);
    qsort(results.items, results.len, sizeof(int), cmp_int);
    printf("[%d, %d, %d]\n", results.items[0], results.items[1], results.items[2]);
    pthread_mutex_unlock(&results.lock);
    return 0;
}
