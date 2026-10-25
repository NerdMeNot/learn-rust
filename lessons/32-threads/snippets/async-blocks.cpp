#include <atomic>
#include <chrono>
#include <future>
#include <iostream>
#include <thread>

int main() {
    std::atomic<bool> finished = false;
    {
        auto future = std::async(std::launch::async, [&] {
            std::this_thread::sleep_for(std::chrono::milliseconds(200));
            finished = true;
        });
    }   // the future is destroyed unused, and its destructor waits for the task
    std::cout << std::boolalpha << "next line: the task has already finished: " << finished << "\n";
}
