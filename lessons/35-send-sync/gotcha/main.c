#include <pthread.h>
#include <stdio.h>
#include <string.h>

struct Job {
    const char *name;
    char *log;      // a buffer shared with main
};

void *run(void *arg) {
    struct Job *job = arg;
    strcat(job->log, job->name);
    return NULL;
}

int main(void) {
    char log[64] = "";
    struct Job job = {"resize", log};
    pthread_t t;
    pthread_create(&t, NULL, run, &job);
    pthread_join(t, NULL);
    printf("log: [\"%s\"]\n", log);
    return 0;
}
