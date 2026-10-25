#include <iostream>
#include <string>

int main() {
    std::string line = "mode";
    auto eq = line.find('=');
    std::cout << line.substr(0, eq) << " -> " << line.substr(eq + 1) << "\n";
}
