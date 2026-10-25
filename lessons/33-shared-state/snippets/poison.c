#include <errno.h>
#include <pthread.h>
#include <stdio.h>

static pthread_mutex_t m;
static int data[4] = {1, 2, 3, 0};

static void *worker(void *arg) {
    (void)arg;
    pthread_mutex_lock(&m);
    data[3] = 4;           // half-done update
    return NULL;           // the thread ends without unlocking
}

int main(void) {
    pthread_mutexattr_t attr;
    pthread_mutexattr_init(&attr);
    pthread_mutexattr_setrobust(&attr, PTHREAD_MUTEX_ROBUST);
    pthread_mutex_init(&m, &attr);

    pthread_t t;
    pthread_create(&t, NULL, worker, NULL);
    pthread_join(t, NULL);

    int rc = pthread_mutex_lock(&m);
    printf("lock returned EOWNERDEAD: %s\n", rc == EOWNERDEAD ? "yes" : "no");
    if (rc == EOWNERDEAD) {
        data[3] = 0;                    // repair the state, then say so
        pthread_mutex_consistent(&m);
    }
    printf("data[3] after repair: %d\n", data[3]);
    pthread_mutex_unlock(&m);
    return 0;
}
