#include <pthread.h>
#include <stdio.h>
#include <unistd.h>

// pipe() is POSIX, not ISO C: bytes in, bytes out, and the reader sees end-of-file once
// every write end is closed, like dropping every Sender.
static int fds[2];

static void *writer(void *arg) {
    (void)arg;
    for (int i = 1; i <= 3; i++) {
        write(fds[1], &i, sizeof i);   // a copy of the int's bytes
    }
    close(fds[1]);                     // the last write end: readers will see EOF
    return NULL;
}

int main(void) {
    pipe(fds);
    pthread_t t;
    pthread_create(&t, NULL, writer, NULL);

    int v;
    while (read(fds[0], &v, sizeof v) == (ssize_t)sizeof v) {
        printf("got %d\n", v);
    }
    printf("read returned 0: every write end is closed\n");
    pthread_join(t, NULL);
    return 0;
}
