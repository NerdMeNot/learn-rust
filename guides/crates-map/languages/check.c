#include <getopt.h>
#include <regex.h>
#include <stdio.h>
#include <time.h>
#include <syslog.h>

int main(int argc, char **argv) {
    regex_t re;
    int ok = regcomp(&re, "[0-9]+-[0-9]+-[0-9]+", REG_EXTENDED) == 0 && regexec(&re, "shipped on 2026-10-25", 0, NULL, 0) == 0;
    regfree(&re);
    int opt = getopt(argc, argv, "v");
    struct tm t = {0};
    t.tm_year = 126; t.tm_mon = 9; t.tm_mday = 25;
    mktime(&t);
    void (*log_fn)(int, const char *, ...) = syslog;
    printf("C: regcomp/regexec %s, getopt returned %d, struct tm weekday %d (0 = Sunday), syslog %s\n", ok ? "matched" : "failed", opt, t.tm_wday, log_fn ? "declared" : "missing");
    return 0;
}
