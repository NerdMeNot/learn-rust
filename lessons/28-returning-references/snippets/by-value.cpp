#include <iostream>
#include <string>

const char *inside = nullptr;

// returns a NEW std::string: the caller owns it
std::string shout(const std::string &name) {
    std::string upper = "HELLO, " + name + ", THIS IS A LONG GREETING";
    inside = upper.data();
    return upper;
}

int main() {
    std::string s = shout("rust");
    std::cout << "s: " << s << "\n";
    std::cout << "same buffer inside and outside: " << std::boolalpha << (s.data() == inside) << "\n";
}
