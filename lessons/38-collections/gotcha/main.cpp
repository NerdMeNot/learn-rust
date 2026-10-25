#include <iostream>
#include <map>
#include <string>

int main() {
    std::map<std::string, unsigned> counts;
    counts["cat"] = 0;
    for (std::string w : {"cat", "cat"}) {
        counts[w] += 1;
    }
    for (const auto &[word, n] : counts) std::cout << word << ": " << n << "\n";
}
