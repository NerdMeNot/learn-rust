#include <pthread.h>
#include <stdio.h>

static int count = 0;

static void *work(void *arg) {
    (void)arg;
    count++; /* no lock, no atomic */
    return NULL;
}

int main(void) {
    pthread_t a, b;
    pthread_create(&a, NULL, work, NULL);
    pthread_create(&b, NULL, work, NULL);
    pthread_join(a, NULL);
    pthread_join(b, NULL);
    printf("count: %d\n", count);
    return 0;
}
