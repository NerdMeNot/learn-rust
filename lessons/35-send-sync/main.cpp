#include <future>
#include <iostream>
#include <string>
#include <vector>

int main() {
    std::vector<std::string> words{"alpha", "beta"};

    // std::async runs the lambda on a new thread and hands back its result later
    auto back = std::async(std::launch::async, [&words] { return words; }).get();
    std::cout << "moved there and back: [\"" << back[0] << "\", \"" << back[1] << "\"]\n";

    // two threads read the same vector at once; nothing checks that this is safe
    auto a = std::async(std::launch::async, [&words] { return words[0].size(); });
    auto b = std::async(std::launch::async, [&words] { return words[1].size(); });
    std::cout << "lengths read in parallel: " << a.get() << " and " << b.get() << "\n";
}
