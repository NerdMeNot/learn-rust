#include <stdio.h>

struct Point {
    int x;
};

struct Team {
    char *name; /* a pointer: copying the struct copies the arrow */
    int scores[2]; /* an array inside a struct is copied with it */
};

static void report(const char *kind, int changed) {
    printf("%-10s %s\n", kind, changed ? "shares: the other name sees the change" : "copy:   the other name is untouched");
}

static void set_first(int arr[], int value) { /* arr is really a pointer */
    arr[0] = value;
}

int main(void) {
    int n = 1;
    int m = n;
    m = 99;
    report("int", n == m);

    struct Point p = {1};
    struct Point q = p;
    q.x = 99;
    report("struct", p.x == q.x);

    struct Point *pp = &p;
    struct Point *pq = pp;
    pq->x = 99;
    report("pointer", p.x == 99);

    int arr[2] = {1, 2};
    set_first(arr, 99); /* passing an array passes its address */
    report("array arg", arr[0] == 99);

    char text[] = "hi";
    struct Team a = {text, {1, 2}};
    struct Team b = a;
    b.name[0] = 'H';
    b.scores[0] = 99;
    printf("struct copy b = a, then b.name[0] = 'H' and b.scores[0] = 99:\n");
    printf("  a.name: %s (shared text), a.scores[0]: %d (own copy)\n", a.name, a.scores[0]);

    /* Identity: compare addresses */
    printf("pp == pq: %d  (two pointers, one address)\n", pp == pq);
    printf("pp == &p: %d  (pp points at p)\n", pp == &p);
    return 0;
}
