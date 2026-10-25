#include <pthread.h>
#include <stdio.h>

static pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;
static long count = 0;

static void *work(void *arg) {
    (void)arg;
    for (int i = 0; i < 1000; i++) {
        pthread_mutex_lock(&lock);
        count++;
        pthread_mutex_unlock(&lock);
    }
    return NULL;
}

int main(void) {
    pthread_t threads[8];
    for (int t = 0; t < 8; t++) {
        pthread_create(&threads[t], NULL, work, NULL);
    }
    for (int t = 0; t < 8; t++) {
        pthread_join(threads[t], NULL);
    }
    printf("count: %ld\n", count);
    return 0;
}
