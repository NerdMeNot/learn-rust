#include <iostream>
#include <string>

// Nothing in this signature says it can throw.
int parse_age(const std::string& s) {
    return std::stoi(s);
}

int main() {
    std::cout << parse_age("abc") << "\n";
}
