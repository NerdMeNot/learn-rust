#define _POSIX_C_SOURCE 200809L
#include <stdio.h>
#include <stdlib.h>
#include <sys/wait.h>
#include <time.h>

int main(int argc, char *argv[]) {
    (void)argv;
    // argc counts the program itself, so one is subtracted
    printf("%d arg(s) after the program name\n", argc - 1);

    // getenv returns NULL when the variable is not set
    const char *port = getenv("RBC_PORT");
    if (port == NULL) port = "8080";
    printf("port %s\n", port);
    const char *missing = getenv("RBC_MISSING");
    if (missing) {
        printf("RBC_MISSING=%s\n", missing);
    } else {
        printf("RBC_MISSING: not set\n");
    }

    // CLOCK_MONOTONIC is not affected when the system clock is set
    struct timespec start, stop;
    clock_gettime(CLOCK_MONOTONIC, &start);
    nanosleep(&(struct timespec){.tv_sec = 0, .tv_nsec = 20 * 1000000L}, NULL);
    clock_gettime(CLOCK_MONOTONIC, &stop);
    long took_ms = (stop.tv_sec - start.tv_sec) * 1000 + (stop.tv_nsec - start.tv_nsec) / 1000000;
    printf("slept at least 20ms: %s\n", took_ms >= 20 ? "true" : "false");

    // popen runs the command through /bin/sh and lets you read its output
    FILE *child = popen("echo 'hello from echo'", "r");
    if (!child) { perror("popen"); return 1; }
    char buf[128];
    while (fgets(buf, sizeof buf, child)) fputs(buf, stdout);
    int status = pclose(child);
    printf("exit ok: %s\n", WIFEXITED(status) && WEXITSTATUS(status) == 0 ? "true" : "false");
    return 0;
}
