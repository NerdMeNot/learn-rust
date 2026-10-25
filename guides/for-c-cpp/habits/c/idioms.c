/* Every C cell of the idiom table on the page. */
#include <errno.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LIMIT 100

static void *work(void *arg) {
    *(int *)arg = 42;
    return NULL;
}

int main(void) {
    {   /* 1 */
        int *p = malloc(sizeof *p);
        *p = 7;
        printf("1 %d\n", *p);
        free(p);
    }
    {   /* 6 */
        const char *s = "hi";
        printf("6 %zu\n", strlen(s));
    }
    {   /* 7 */
        const char *p = strchr("mode", '=');
        if (p != NULL) printf("7 found\n");
        else printf("7 NULL\n");
    }
    {   /* 9 */
        const char *text = "abc";
        char *end;
        errno = 0;
        long n = strtol(text, &end, 10);
        if (end == text) printf("9 no digits (n=%ld)\n", n);
    }
    {   /* 17 */
        int result = 0;
        pthread_t t;
        pthread_create(&t, NULL, work, &result);
        pthread_join(t, NULL);
        printf("17 %d\n", result);
    }
    {   /* 18 */
        double x = 3.9;
        printf("18 %d\n", (int)x);
    }
    printf("19 %d\n", LIMIT);
    {   /* 20 */
        int x = 5;
        int *p = &x;
        printf("20 %d\n", *p);
    }
    return 0;
}
