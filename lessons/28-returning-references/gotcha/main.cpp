#include <iostream>
#include <string>

const std::string& make_greeting(const std::string& name) {
    std::string s = "hello " + name;
    return s;
}

int main() {
    std::cout << make_greeting("rust") << "\n";
}
