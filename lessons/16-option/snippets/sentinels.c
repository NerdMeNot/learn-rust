#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    // "No result" is a different convention in every function.
    printf("atoi(\"0\"): %d\n", atoi("0"));
    printf("atoi(\"zero\"): %d\n", atoi("zero"));
    printf("strchr(\"hello\", 'z') is NULL: %d\n", strchr("hello", 'z') == NULL);
    printf("strcmp(\"a\", \"a\"): %d\n", strcmp("a", "a"));
    printf("getenv(\"NO_SUCH_VAR\") is NULL: %d\n", getenv("NO_SUCH_VAR") == NULL);
    return 0;
}
