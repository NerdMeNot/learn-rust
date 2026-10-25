#include <iostream>
#include <thread>
#include <vector>

int main() {
    std::vector<int> results;
    std::vector<std::thread> threads;
    for (int i = 0; i < 3; ++i) {
        threads.emplace_back([&results, i] {
            results.push_back(i * 10);   // no lock!
        });
    }
    for (auto& t : threads) {
        t.join();
    }
    std::cout << results.size() << "\n";
}
