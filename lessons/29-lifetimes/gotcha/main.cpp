#include <iostream>
#include <string>

const std::string &longest(const std::string &a, const std::string &b) {
    return a.size() >= b.size() ? a : b;
}

int main() {
    std::string s1 = "a long first string";
    const std::string *result;
    {
        std::string s2 = "short";
        result = &longest(s1, s2);
    }
    std::cout << "longest: " << *result << "\n";
}
