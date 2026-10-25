// The same trap as in Rust: a loop with a formula.
unsigned long sum_to(unsigned long n) {
    unsigned long total = 0;
    for (unsigned long i = 0; i < n; i++)
        total += i;
    return total;
}
