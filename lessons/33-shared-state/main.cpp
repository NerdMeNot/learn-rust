#include <algorithm>
#include <iostream>
#include <mutex>
#include <thread>
#include <vector>

int main() {
    std::vector<int> results;
    std::mutex m;

    {
        std::vector<std::jthread> threads;
        for (int i = 0; i < 3; ++i) {
            threads.emplace_back([&results, &m, i] {
                std::lock_guard lock(m);      // lock, push, unlock at the closing brace
                results.push_back(i * 10);
            });
        }
    }   // jthreads join here, so the references above never outlive results

    std::ranges::sort(results);
    std::cout << "[";
    for (std::size_t i = 0; i < results.size(); ++i) {
        std::cout << (i ? ", " : "") << results[i];
    }
    std::cout << "]\n";
}
