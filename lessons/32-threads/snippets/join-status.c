#include <errno.h>
#include <pthread.h>
#include <stdio.h>
#include <string.h>

void *worker(void *arg) {
    (void)arg;
    return (void *)42;                       // the thread's value travels as a void *
}

int main(void) {
    pthread_t t;
    pthread_create(&t, NULL, worker, NULL);
    void *result;
    int status = pthread_join(t, &result);   // the int is about the join call itself
    printf("join status: %d, the thread's value: %ld\n", status, (long)result);

    status = pthread_join(pthread_self(), NULL); // a join that cannot work
    printf("joining myself: status %d (%s)\n", status, strerror(status));
    return 0;
}
