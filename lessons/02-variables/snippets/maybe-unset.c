#include <stdio.h>

int main(int argc, char **argv) {
    (void)argv;
    int label;
    if (argc > 5) {
        label = 1;
    }
    printf("label: %d\n", label); // the branch did not run: label was never set
    return 0;
}
