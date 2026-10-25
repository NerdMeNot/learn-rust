#include <atomic>
#include <chrono>
#include <future>
#include <iostream>
#include <thread>

int main() {
    std::atomic<bool> ran{false};
    // std::async starts the work on another thread right away
    std::future<int> f = std::async(std::launch::async, [&] {
        ran = true;
        return 42;
    });
    while (f.wait_for(std::chrono::milliseconds(1)) != std::future_status::ready) {
    }
    std::cout << "the body ran before anyone called get(): " << std::boolalpha << ran << "\n";
    std::cout << "get() returns " << f.get() << "\n";
}
