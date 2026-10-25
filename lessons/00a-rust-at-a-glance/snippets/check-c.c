#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

int add(int a, int b) { return a + b; }
int inc(int x) { return x + 1; }

int parse(const char *s, int *out) {
    char *end;
    long v = strtol(s, &end, 10);
    if (*s == '\0' || *end != '\0') return -1;
    *out = (int)v;
    return 0;
}

int double_parsed(const char *s, int *out) {
    int n;
    if (parse(s, &n) != 0) return -1;
    *out = n * 2;
    return 0;
}

int main(void) {
    int x = 5;
    x = 6;
    const int y = 5;
    assert(x == 6 && y == 5);
    assert(add(2, 3) == 5);

    int count = 0;
    for (int i = 0; i < 3; i++) {
        count += i;
    }
    assert(count == 3);

    const char *words[] = {"a", "b"};
    int total = 0;
    for (size_t i = 0; i < 2; i++) {
        total += (int)words[i][0];
    }
    assert(total == 'a' + 'b');

    int n = 1;
    const char *s;
    switch (n) {
    case 1:
        s = "one";
        break;
    default:
        s = "many";
    }
    assert(s[0] == 'o');

    int (*f)(int) = inc;
    assert(f(1) == 2);

    const char *name = "Ana";
    int age = 30;
    printf("%s is %d\n", name, age);

    int out;
    assert(double_parsed("21", &out) == 0 && out == 42);
    assert(double_parsed("x", &out) == -1);
    puts("all c cells compile and hold");
    return 0;
}
