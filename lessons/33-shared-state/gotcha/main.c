#include <pthread.h>
#include <stdio.h>

int results[3];
int len = 0;

void *worker(void *arg) {
    int i = *(int *)arg;
    results[len++] = i * 10;   // no lock!
    return NULL;
}

int main(void) {
    pthread_t threads[3];
    int ids[3];
    for (int i = 0; i < 3; i++) {
        ids[i] = i;
        pthread_create(&threads[i], NULL, worker, &ids[i]);
    }
    for (int i = 0; i < 3; i++) {
        pthread_join(threads[i], NULL);
    }
    printf("%d\n", len);
    return 0;
}
