#include <iostream>
#include <string>
#include <vector>

int main() {
    std::vector<std::string> names{"ana", "bo"};
    for (auto n : names) {
        std::cout << "hi " << n << "\n";
    }
    std::cout << names.size() << " names\n";
}
