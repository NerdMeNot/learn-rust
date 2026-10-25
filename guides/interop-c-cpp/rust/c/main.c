#include <stdio.h>
#include <stdlib.h>
#include "bridge.h"

int main(void) {
    Point a = {0.0, 0.0}, b = {3.0, 4.0};
    printf("distance: %.1f\n", point_distance(a, b));

    Reading r = {1, 2.5, 0};
    printf("reading: %.1f\n", reading_value(&r));

    int numbers[] = {10, 20, 30};
    printf("sum: %lld\n", (long long)sum_i32(numbers, 3));

    char *text = greeting("C");
    printf("%s\n", text);
    greeting_free(text);

    Accumulator *acc = accumulator_new();
    accumulator_add(acc, 5);
    printf("total: %lld\n", (long long)accumulator_add(acc, 6));
    accumulator_free(acc);
    return 0;
}
