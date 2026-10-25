#include <algorithm>
#include <execution>
#include <iostream>
#include <mutex>
#include <numeric>
#include <set>
#include <thread>
#include <vector>

int main() {
    std::vector<long long> numbers(1'000'000);
    std::iota(numbers.begin(), numbers.end(), 1);

    std::set<std::thread::id> threads; // which threads ran the lambda
    std::mutex m;
    long long total = std::transform_reduce(
        std::execution::par, numbers.begin(), numbers.end(), 0LL, std::plus<>(),
        [&](long long n) {
            std::lock_guard<std::mutex> lock(m);
            threads.insert(std::this_thread::get_id());
            return n * n;
        });
    std::cout << total << ", threads that ran the lambda: " << threads.size() << "\n";
}
