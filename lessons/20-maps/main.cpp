#include <iostream>
#include <ranges>
#include <string>
#include <string_view>
#include <unordered_map>

int main() {
    std::unordered_map<std::string, unsigned> ages;
    ages["ana"] = 30;
    ages["bo"] = 25;

    // find gives an iterator: end() means "not there"
    if (auto it = ages.find("ana"); it != ages.end()) {
        std::cout << "ana is " << it->second << "\n";
    } else {
        std::cout << "no ana\n";
    }

    // a default on purpose: 0 when the key is missing (without inserting it)
    auto it = ages.find("zed");
    unsigned zed = it != ages.end() ? it->second : 0;
    std::cout << "zed is " << zed << "\n";

    // count words: operator[] inserts 0 first when the key is new
    std::unordered_map<std::string_view, unsigned> counts;
    for (auto word : std::string_view("a b a c a") | std::views::split(' ')) {
        counts[std::string_view(word.begin(), word.end())] += 1;
    }
    std::cout << "a appears " << counts.at("a") << " times\n";
}
