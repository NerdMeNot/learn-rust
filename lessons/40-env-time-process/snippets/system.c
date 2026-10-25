#include <stdio.h>
#include <stdlib.h>
#include <sys/wait.h>

int main(void) {
    const char *user = "world; echo injected";
    char cmd[200];
    snprintf(cmd, sizeof cmd, "echo hello %s", user);
    system(cmd); /* the string goes to /bin/sh, so the semicolon starts a second command */

    int status = system("exit 3");
    printf("exit 3: system returned %d; WEXITSTATUS gives %d\n", status, WEXITSTATUS(status));

    status = system("kill -9 $$");
    printf("kill -9: system returned %d; WIFSIGNALED %d, WTERMSIG %d\n", status,
           WIFSIGNALED(status) != 0, WTERMSIG(status));
    return 0;
}
