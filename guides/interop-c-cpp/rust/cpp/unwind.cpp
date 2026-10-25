#include <cstdio>
#include "bridge.h"

int main() {
    try {
        explode_unwind();
        std::puts("not reached");
    } catch (...) {
        std::puts("C++ caught what Rust threw");
        std::fflush(stdout); /* so the line survives the abort even when output is piped */
    }
    return 0;
}
