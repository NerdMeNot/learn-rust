#include <cstdio>

int main(int argc, char **argv) {
    (void)argv;
    int x = argc + 4;
    {
        int x = x + 1; // this x is already in scope on the right: it reads itself
        printf("inner x: %d\n", x);
    }
    printf("outer x: %d\n", x);
    return 0;
}
