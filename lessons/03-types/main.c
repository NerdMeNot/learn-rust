#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>

int main(void) {
    int32_t a = 7;        // 32-bit signed integer
    int64_t b = 10;       // 64-bit signed integer
    double price = 2.5;   // 64-bit float
    bool ok = true;

    int64_t total = (int64_t)a + b;   // a cast, like Rust's `as`
    double cost = (double)a * price;
    printf("%lld %g %s\n", (long long)total, cost, ok ? "true" : "false");
    return 0;
}
