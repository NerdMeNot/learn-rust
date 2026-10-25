#include <iostream>
#include <string>

int main() {
    std::string cmd = "go";
    switch (cmd) {
    case "go":
        std::cout << "moving\n";
        break;
    default:
        std::cout << "unknown command\n";
    }
}
