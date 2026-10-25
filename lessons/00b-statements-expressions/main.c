#include <stdio.h>

int clamp(int x) {
    if (x < 0) {
        return 0;
    }
    return x > 100 ? 100 : x;
}

int main(void) {
    int n = 7;
    const char *label = n % 2 == 0 ? "even" : "odd";
    int w = 3;
    int h = 4;
    int area = w * h;
    printf("%d is %s\n", n, label);
    printf("area: %d\n", area);
    printf("clamp: %d %d %d\n", clamp(-5), clamp(50), clamp(250));
    return 0;
}
