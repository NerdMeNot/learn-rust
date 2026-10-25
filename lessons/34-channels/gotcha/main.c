#define _POSIX_C_SOURCE 200809L  // for nanosleep
#include <pthread.h>
#include <stdio.h>
#include <time.h>

// a one-slot "channel": a pointer handed over under a lock
pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;
pthread_cond_t ready = PTHREAD_COND_INITIALIZER;
int *slot = NULL;

void sleep_ms(long ms) {
    struct timespec ts = {0, ms * 1000000L};
    nanosleep(&ts, NULL);
}

void *receiver(void *arg) {
    (void)arg;
    pthread_mutex_lock(&lock);
    while (slot == NULL) pthread_cond_wait(&ready, &lock);
    int *got = slot;
    pthread_mutex_unlock(&lock);

    sleep_ms(10);
    printf("receiver sees [%d, %d]\n", got[0], got[1]);
    return NULL;
}

int main(void) {
    pthread_t t;
    pthread_create(&t, NULL, receiver, NULL);

    int msg[2] = {1, 2};
    pthread_mutex_lock(&lock);
    slot = msg;                  // send
    pthread_cond_signal(&ready);
    pthread_mutex_unlock(&lock);

    msg[0] = 99;                 // sender keeps changing what it already sent
    pthread_join(t, NULL);
    return 0;
}
