#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

// A pipe is a bounded channel of bytes with back-pressure and a hang-up error.
int main(void) {
    int fds[2];
    if (pipe(fds) != 0) return 1;
    printf("pipe capacity: %d bytes\n", fcntl(fds[1], F_GETPIPE_SZ));

    fcntl(fds[1], F_SETFL, O_NONBLOCK);   // a full pipe now fails instead of waiting
    char byte = 'x';
    long total = 0;
    while (write(fds[1], &byte, 1) == 1) total++;
    printf("bytes written before the pipe was full: %ld (%s)\n", total, strerror(errno));

    signal(SIGPIPE, SIG_IGN);             // otherwise the write below kills the process
    close(fds[0]);                        // the receiver leaves
    ssize_t n = write(fds[1], &byte, 1);
    printf("write with no reader: %zd (%s)\n", n, strerror(errno));
    return 0;
}
