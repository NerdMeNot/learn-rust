#include <iostream>
#include <map>
#include <string>

int main() {
    // An initializer list is part of the language: no macro needed
    std::map<std::string, int> ages{{"Ada", 36}, {"Linus", 28}, {"Grace", 45}};

    for (const auto &[name, age] : ages) { // std::map iterates in key order
        std::cout << name << ": " << age << "\n";
    }
    std::cout << "entries: " << ages.size() << "\n";
    return 0;
}
