#include <iostream>
#include <thread>

int main() {
    std::thread t([] {});
    std::cout << "main: leaving without join() or detach()\n";
}   // t is destroyed while still joinable: std::terminate
