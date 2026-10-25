#include <iostream>

void log_msg(const char *msg) {
    std::cout << "log: " << msg << "\n";
}

int main() {
    auto r = log_msg("hello");
    (void)r;
}
