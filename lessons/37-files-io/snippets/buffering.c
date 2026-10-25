#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* how many write system calls this process has made so far (Linux: the syscw line) */
static long write_calls(void) {
    FILE *f = fopen("/proc/self/io", "r");
    char line[128];
    long n = -1;
    while (fgets(line, sizeof line, f))
        if (strncmp(line, "syscw:", 6) == 0) n = atol(line + 6);
    fclose(f);
    return n;
}

int main(void) {
    /* standard output redirected to a file: C switches from line to full buffering */
    freopen("/tmp/lr-buffering-c.txt", "w", stdout);
    long before = write_calls();
    for (int i = 0; i < 1000; i++) printf("line %d\n", i);
    long waiting = write_calls() - before;
    fflush(stdout);
    long done = write_calls() - before;
    fprintf(stderr, "1000 printf: %ld write calls before fflush, %ld after\n", waiting, done);
    remove("/tmp/lr-buffering-c.txt");
    return 0;
}
