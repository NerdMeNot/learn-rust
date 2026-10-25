#include <format>
#include <iostream>
#include <string>
#include <string_view>

// returns a NEW std::string: the caller owns it
std::string make_greeting(std::string_view name) {
    return std::format("hello {}", name);
}

// returns a view into the input: no copy, only valid while `text` lives
std::string_view first_word(std::string_view text) {
    return text.substr(0, text.find(' '));
}

int main() {
    std::string g = make_greeting("rust");
    std::string_view w = first_word(g);
    std::cout << g << " / " << w << "\n";
}
