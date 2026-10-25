#include <stdio.h>
#include <stdlib.h>

struct Buf {
    char *data;
};

int main(void) {
    struct Buf a = {malloc(16)};
    struct Buf b = a; // copies the pointer, not the memory
    free(a.data);
    printf("freed once\n");
    free(b.data);
    printf("freed twice\n");
}
