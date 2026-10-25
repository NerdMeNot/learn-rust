#include <iostream>
#include <map>
#include <string>
#include <unordered_map>

int main() {
    std::unordered_map<std::string, int> stock;
    std::cout << "size before: " << stock.size() << "\n";
    int zed = stock["zed"];  // only reading, or so it looks
    std::cout << "after stock[\"zed\"]: size " << stock.size() << ", value " << zed << "\n";

    std::cout << "contains(\"zed\"): " << stock.contains("zed") << "\n";

    try {
        stock.at("nope");
    } catch (const std::out_of_range &) {
        std::cout << "at(\"nope\"): std::out_of_range\n";
    }

    std::unordered_map<std::string, int> hashed{{"one", 1}, {"two", 2}, {"three", 3}, {"four", 4}};
    std::map<std::string, int> sorted(hashed.begin(), hashed.end());
    std::cout << "unordered_map:";
    for (const auto &[k, v] : hashed) std::cout << " " << k;
    std::cout << "\nmap:";
    for (const auto &[k, v] : sorted) std::cout << " " << k;
    std::cout << "\n";
}
