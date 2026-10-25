#include <future>
#include <iostream>
#include <string>
#include <utility>
#include <vector>

int main() {
    std::vector<std::string> names{"a", "b", "c"};

    // std::async runs the lambda on a new thread and hands back its result later
    auto result = std::async(std::launch::async, [names = std::move(names)] {   // the thread takes names
        for (const auto& n : names) {
            std::cout << "worker sees " << n << "\n";
        }
        return names.size();
    });

    auto count = result.get();   // wait for the thread
    std::cout << "worker counted " << count << "\n";
}
