#include <dirent.h>
#include <errno.h>
#include <stdio.h>
#include <string.h>

/* how many descriptors this process has open (Linux) */
static int open_fds(void) {
    int n = 0;
    DIR *d = opendir("/proc/self/fd");
    while (readdir(d)) n++;
    closedir(d);
    return n;
}

/* f is gone when this returns, but nothing closed it */
static void leak(const char *path) {
    FILE *f = fopen(path, "r");
    (void)f;
}

int main(void) {
    FILE *missing = fopen("/no/such/file.txt", "r");
    if (missing == NULL) printf("fopen failed: %s (errno %d)\n", strerror(errno), errno);

    char path[] = "/tmp/lr-fio-c.txt";
    FILE *w = fopen(path, "w");
    fputs("x", w);
    fclose(w);

    int base = open_fds();
    leak(path);
    printf("after leak() returned: %d more open\n", open_fds() - base);

    /* fclose flushes the buffer, and its return value reports a failure there.
       /dev/full refuses every byte, like a full disk. */
    FILE *full = fopen("/dev/full", "w");
    fputs("data", full);
    int rc = fclose(full);
    int err = errno;
    printf("fclose on /dev/full returned %d (%s)\n", rc, strerror(err));
    remove(path);
    return 0;
}
