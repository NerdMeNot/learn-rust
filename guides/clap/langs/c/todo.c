#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

int main(int argc, char **argv) {
    int verbose = 0;
    long priority = 2;
    int c;
    while ((c = getopt(argc, argv, "vp:")) != -1) {
        switch (c) {
        case 'v':
            verbose = 1;
            break;
        case 'p': {
            char *end;
            priority = strtol(optarg, &end, 10);
            if (*optarg == '\0' || *end != '\0') {
                fprintf(stderr, "invalid priority: %s\n", optarg);
                return 2;
            }
            break;
        }
        default:
            return 2;
        }
    }
    if (optind < argc && strcmp(argv[optind], "list") == 0) {
        puts("nothing to list yet");
    } else if (optind + 1 < argc && strcmp(argv[optind], "add") == 0) {
        printf("added \"%s\" with priority %ld\n", argv[optind + 1], priority);
        if (verbose) puts("(verbose mode is on)");
    } else {
        fprintf(stderr, "usage: todo [-v] add [-p N] TEXT | list\n");
        return 2;
    }
    return 0;
}
