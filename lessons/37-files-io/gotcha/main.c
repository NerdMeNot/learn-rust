#include <stdio.h>

int main(void) {
    FILE *f = fopen("/tmp/rbc-hello.txt", "w");
    fputs("hello\n", f);
    fclose(f);

    f = fopen("/tmp/rbc-hello.txt", "r");
    char text[64];
    fgets(text, sizeof text, f);
    text[strcspn(text, "\n")] = '\0';
    printf("%s (%zu bytes)\n", text, strlen(text));
    fclose(f);
    remove("/tmp/rbc-hello.txt");
    return 0;
}
