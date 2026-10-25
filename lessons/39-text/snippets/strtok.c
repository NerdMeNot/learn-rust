#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    char text[] = "a,,b";
    printf("before: \"a,,b\" has %zu bytes\n", strlen(text));
    int pieces = 0;
    for (char *tok = strtok(text, ","); tok != NULL; tok = strtok(NULL, ",")) {
        printf("piece: \"%s\"\n", tok);
        pieces++;
    }
    printf("pieces: %d\n", pieces);
    printf("after: the array now holds \"%s\" (length %zu)\n", text, strlen(text));

    // strtok_r keeps its position in a variable you own, but it still writes into the text.
    char again[] = "a,,b";
    char *save = NULL;
    for (char *tok = strtok_r(again, ",", &save); tok != NULL; tok = strtok_r(NULL, ",", &save)) {
        printf("strtok_r piece: \"%s\"\n", tok);
    }
    printf("after strtok_r: the array holds \"%s\"\n", again);

    char *end;
    long n = strtol("42abc", &end, 10);
    printf("strtol(\"42abc\"): %ld, rest \"%s\"\n", n, end);
    printf("atoi(\"abc\"): %d\n", atoi("abc"));
    printf("atoi(\"99999999999\"): %d\n", atoi("99999999999"));
    return 0;
}
