#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    const char *owned = "héllo";       // bytes ending in '\0' (UTF-8)
    // a "view" is a pointer plus a length; %.*s prints that many bytes
    const char *view = owned;
    int view_len = 1;

    // a growable string: allocate room, then append
    char *s = malloc(32);
    strcpy(s, "go");
    strcat(s, " + rust");

    printf("%s %.*s %s\n", owned, view_len, view, s);

    // count characters: skip UTF-8 continuation bytes (10xxxxxx)
    size_t chars = 0;
    for (const char *p = owned; *p; p++) {
        if (((unsigned char)*p & 0xC0) != 0x80) chars++;
    }
    printf("bytes=%zu chars=%zu\n", strlen(owned), chars);

    // print each character: a lead byte plus its continuation bytes
    for (const char *p = owned; *p;) {
        const char *start = p++;
        while (((unsigned char)*p & 0xC0) == 0x80) p++;
        printf("[%.*s]", (int)(p - start), start);
    }
    printf("\n");
    free(s);
    return 0;
}
