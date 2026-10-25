#include <stdio.h>

int add(int a, int b) {
    return a + b;            // return gives the value back
}

void greet(const char *name) {   // void means it returns nothing
    printf("hi %s\n", name);
}

int main(void) {
    printf("%d\n", add(2, 3));
    greet("kotlin");
    return 0;
}
