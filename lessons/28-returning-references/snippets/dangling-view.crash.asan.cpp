#include <iostream>
#include <string>
#include <string_view>

std::string_view first_word(const std::string& text) {
    return std::string_view(text).substr(0, text.find(' '));
}

int main() {
    // the temporary string is destroyed at the semicolon; it is long enough to need the heap
    std::string_view w = first_word(std::string("hello world, this is a long text"));
    std::cout << "w: " << w << "\n";
}
