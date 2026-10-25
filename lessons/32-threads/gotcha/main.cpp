#include <iostream>
#include <string>
#include <thread>
#include <vector>

int main() {
    std::vector<std::string> names{"a", "b", "c"};
    std::thread t([&] {
        std::cout << names.size() << "\n";
    });
    t.join();
}
