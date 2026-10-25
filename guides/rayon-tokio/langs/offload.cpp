#include <future>
#include <iostream>

long work() {
    long total = 0;
    for (long n = 0; n < 1000000; n++) total += n % 7;
    return total;
}

int main() {
    auto f = std::async(std::launch::async, work); // a thread, and a future to wait on
    std::cout << "std::async: " << f.get() << "\n";
}
