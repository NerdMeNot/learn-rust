#include <stdio.h>
#include <string.h>

static int bad = 0;

static void same(const char *got, const char *want) {
    if (strcmp(got, want) != 0) {
        printf("got [%s], want [%s]\n", got, want);
        bad = 1;
    }
}

int main(void) {
    char b[64];
    snprintf(b, sizeof b, "%5d", 42);
    same(b, "   42");
    snprintf(b, sizeof b, "%-5d", 42);
    same(b, "42   ");
    snprintf(b, sizeof b, "%05d", 42);
    same(b, "00042");
    snprintf(b, sizeof b, "%+d", 42);
    same(b, "+42");
    snprintf(b, sizeof b, "%.2f", 3.14159);
    same(b, "3.14");
    snprintf(b, sizeof b, "%8.3f", 3.14159);
    same(b, "   3.142");
    snprintf(b, sizeof b, "%x", 255);
    same(b, "ff");
    snprintf(b, sizeof b, "%#x", 255);
    same(b, "0xff");
    snprintf(b, sizeof b, "%10s", "Ada");
    same(b, "       Ada");
    snprintf(b, sizeof b, "%-10s", "Ada");
    same(b, "Ada       ");
    snprintf(b, sizeof b, "%.3s", "Lovelace");
    same(b, "Lov");
    snprintf(b, sizeof b, "%d %s", 42, "Ada");
    same(b, "42 Ada");
    if (!bad) printf("ok\n");
    return bad;
}
