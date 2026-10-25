#include <atomic>
#include <iostream>
#include <mutex>
#include <thread>
#include <vector>

int main() {
    std::mutex m;
    long count = 0;               // a separate variable; nothing ties m to count
    std::atomic<long> hits{0};
    std::vector<std::thread> threads;
    for (int t = 0; t < 8; t++) {
        threads.emplace_back([&] {
            for (int i = 0; i < 1000; i++) {
                {
                    std::lock_guard<std::mutex> guard(m);
                    count++;
                }
                hits++;
            }
        });
    }
    for (auto& t : threads) {
        t.join();
    }
    std::cout << "count: " << count << ", hits: " << hits << "\n";
}
