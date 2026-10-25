#include <iostream>
#include <map>
#include <string>

int main() {
    std::map<std::string, int> ages{{"Ada", 36}, {"Linus", 28}};
    std::cout << "entries: " << ages.size() << "\n";
    return 0;
}
