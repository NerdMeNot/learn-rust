#include <pthread.h>
#include <stdio.h>

static void *work(void *out) {
    long total = 0;
    for (long n = 0; n < 1000000; n++) total += n % 7;
    *(long *)out = total;
    return NULL;
}

int main(void) {
    pthread_t t;
    long total = 0;
    pthread_create(&t, NULL, work, &total);
    pthread_join(t, NULL);
    printf("pthread: %ld\n", total);
    return 0;
}
