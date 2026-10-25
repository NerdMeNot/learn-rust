#include <future>
#include <iostream>

std::future<int> load_count() {
    return std::async(std::launch::async, [] { return 3; });
}

int report() {
    int n = load_count();
    return n + 1;
}

int main() {
    std::cout << "count: " << report() << "\n";
}
