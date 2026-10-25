#include <iostream>
#include <string>
#include <vector>

int main() {
    std::vector<std::string> names{"ana", "bo"};
    std::string first = names[0];   // copies the whole string
    std::cout << first << " of " << names.size() << "\n";
}
