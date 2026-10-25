#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    const char *tmp = getenv("TMPDIR");
    char path[512];
    snprintf(path, sizeof path, "%s/rbc-notes.txt", tmp ? tmp : "/tmp");

    // "w" creates the file, or empties it first
    FILE *f = fopen(path, "w");
    if (!f) { perror(path); return 1; }
    fputs("milk\neggs\n", f);
    fclose(f);

    // "a" opens for appending
    f = fopen(path, "a");
    if (!f) { perror(path); return 1; }
    fprintf(f, "bread\n");
    fclose(f);

    // C has no "read the whole file": find the size, then fread it all
    f = fopen(path, "r");
    if (!f) { perror(path); return 1; }
    fseek(f, 0, SEEK_END);
    long size = ftell(f);
    rewind(f);
    char *all = malloc((size_t)size + 1);
    size_t n = fread(all, 1, (size_t)size, f);
    all[n] = '\0';
    printf("%zu bytes\n", n);
    free(all);

    // read line by line: fgets keeps the '\n', so cut it off
    rewind(f);
    char line[128];
    for (int i = 1; fgets(line, sizeof line, f); i++) {
        line[strcspn(line, "\n")] = '\0';
        printf("%d: %s\n", i, line);
    }
    fclose(f);

    printf("file name: \"%s\"\n", strrchr(path, '/') + 1);

    // a missing file gives NULL, and errno says why
    FILE *missing = fopen("/no/such/file.txt", "r");
    if (!missing) {
        printf("error: %s (not found: %s)\n", strerror(errno), errno == ENOENT ? "true" : "false");
    } else {
        fclose(missing);
    }

    remove(path);
    return 0;
}
