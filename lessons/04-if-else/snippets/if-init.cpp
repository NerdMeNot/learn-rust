#include <iostream>
#include <map>
#include <string>

int main() {
    std::map<std::string, int> ages{{"ana", 31}};
    // C++17: the declaration before `;` is scoped to the if/else
    if (auto it = ages.find("ana"); it != ages.end()) {
        std::cout << "found " << it->second << "\n";
    } else {
        std::cout << "missing\n";
    }
    return 0;
}
