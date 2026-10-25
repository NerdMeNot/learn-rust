#include <pthread.h>
#include <stdio.h>

struct Job {
    const char *names[3];
    size_t len;
    size_t count;   // the thread writes its answer here
};

void *worker(void *arg) {
    struct Job *job = arg;
    for (size_t i = 0; i < job->len; i++) {
        printf("worker sees %s\n", job->names[i]);
    }
    job->count = job->len;
    return NULL;
}

int main(void) {
    struct Job job = {{"a", "b", "c"}, 3, 0};

    pthread_t t;
    pthread_create(&t, NULL, worker, &job);
    pthread_join(t, NULL);              // wait for the thread

    printf("worker counted %zu\n", job.count);
    return 0;
}
