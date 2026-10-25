#include <pthread.h>
#include <stdio.h>

static pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t changed = PTHREAD_COND_INITIALIZER;
static int started = 0, go = 0;

void *worker(void *arg) {
    (void)arg;
    pthread_mutex_lock(&lock);
    started = 1;
    pthread_cond_broadcast(&changed);
    while (!go) pthread_cond_wait(&changed, &lock);
    pthread_mutex_unlock(&lock);
    printf("worker: still running after main called pthread_exit\n");
    return NULL;
}

int main(void) {
    pthread_t t;
    pthread_create(&t, NULL, worker, NULL);
    pthread_mutex_lock(&lock);
    while (!started) pthread_cond_wait(&changed, &lock);
    printf("main: calling pthread_exit instead of returning\n");
    go = 1;
    pthread_cond_broadcast(&changed);
    pthread_mutex_unlock(&lock);
    pthread_exit(NULL);                      // main ends; the process lives on
}
