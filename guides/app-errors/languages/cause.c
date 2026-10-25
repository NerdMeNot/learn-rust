#include <errno.h>
#include <stdio.h>
#include <string.h>

int main(void) {
    FILE *f = fopen("missing.conf", "r");
    if (f == NULL) {
        /* the callee reports only a code; adding context is up to the caller */
        printf("fopen failed, errno=%d: %s\n", errno, strerror(errno));
        return 1;
    }
    fclose(f);
    return 0;
}
