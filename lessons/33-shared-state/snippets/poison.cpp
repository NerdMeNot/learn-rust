#include <iostream>
#include <mutex>
#include <stdexcept>
#include <thread>
#include <vector>

int main() {
    std::vector<int> data{1, 2, 3};
    std::mutex m;

    std::thread worker([&] {
        try {
            std::lock_guard<std::mutex> guard(m);
            data.push_back(4);
            throw std::runtime_error("worker failed halfway");
        } catch (const std::runtime_error& e) {
            std::cout << "worker died: " << e.what() << "\n";
        }
    });
    worker.join();

    std::lock_guard<std::mutex> guard(m);  // not poisoned: unwinding released the lock
    std::cout << "main got the lock; data:";
    for (int x : data) {
        std::cout << " " << x;
    }
    std::cout << "\n";
}
