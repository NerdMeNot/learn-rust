#include <iostream>
#include <stdexcept>
#include <thread>

int main() {
    std::thread t([] { throw std::runtime_error("worker failed"); });
    t.join();
    std::cout << "main: never reached\n";
}
