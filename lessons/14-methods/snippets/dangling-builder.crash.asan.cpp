#include <iostream>
#include <string>

struct Builder {
    std::string name;
    Builder &set_name(const std::string &n) {
        name = n;
        return *this;
    }
};

int main() {
    // The Builder() temporary is destroyed at the end of this statement.
    auto &b = Builder().set_name("a name too long for the small-string buffer");
    std::cout << b.name << "\n";
}
