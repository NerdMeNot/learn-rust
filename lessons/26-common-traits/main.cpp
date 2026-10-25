#include <algorithm>
#include <compare>
#include <iostream>
#include <set>
#include <string>
#include <vector>

// A defaulted <=> writes <, > and the rest from the members in order; a defaulted ==
// writes == (and != follows from it).
struct Version {
    unsigned major = 0;
    unsigned minor = 0;
    auto operator<=>(const Version &) const = default;
    bool operator==(const Version &) const = default;
};

// operator<< is the user-facing text; nothing prints a struct otherwise.
std::ostream &operator<<(std::ostream &os, const Version &v) {
    return os << "v" << v.major << "." << v.minor;
}

int main() {
    Version a{1, 10};
    Version b{1, 9};
    std::cout << "display: " << a << "\n";
    std::cout << "a == b: " << std::boolalpha << (a == b) << "\n";
    std::cout << "a > b: " << (a > b) << "\n";

    std::vector<Version> all{a, b, Version{}};
    std::sort(all.begin(), all.end());
    std::cout << "sorted:";
    for (const Version &v : all) std::cout << " " << v;
    std::cout << "\n";

    // std::set orders with operator<; std::unordered_set would also need a std::hash.
    std::set<Version> seen;
    for (const Version &v : {a, b, Version{}, a}) seen.insert(v);
    std::cout << "distinct: " << seen.size() << "\n";
}
