#include <iostream>
#include <mutex>
#include <thread>

int main() {
    std::mutex m;
    std::unique_lock<std::mutex> guard(m); // locked on the main thread

    // moving the guard into another thread compiles; unlocking there is the bug
    std::thread t([g = std::move(guard)]() mutable { g.unlock(); });
    t.join();
    std::cout << "done\n";
}
