#include <iostream>
#include <string>
#include <unordered_map>

int main() {
    std::unordered_map<std::string, unsigned> ages{{"ana", 30}};
    unsigned age = ages["ana"];
    std::cout << age << "\n";
}
