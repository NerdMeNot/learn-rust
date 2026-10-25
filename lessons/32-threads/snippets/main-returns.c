#include <pthread.h>
#include <stdio.h>
#include <unistd.h>

static pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t started = PTHREAD_COND_INITIALIZER;
static int running = 0;

void *worker(void *arg) {
    (void)arg;
    pthread_mutex_lock(&lock);
    running = 1;
    pthread_cond_signal(&started);
    pthread_mutex_unlock(&lock);
    pause();                                  // sleeps until a signal arrives
    printf("worker: woke up\n");
    return NULL;
}

int main(void) {
    pthread_t t;
    pthread_create(&t, NULL, worker, NULL);   // never joined
    pthread_mutex_lock(&lock);
    while (!running) pthread_cond_wait(&started, &lock);
    pthread_mutex_unlock(&lock);
    printf("main: the worker is running, and main returns now\n");
    return 0;                                 // ends the whole process
}
