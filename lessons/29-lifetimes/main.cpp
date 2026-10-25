#include <iostream>
#include <string>
#include <string_view>

// Nothing in the types says the result points into a or b. Both are views of
// someone else's characters, and keeping them valid is up to you.
std::string_view longest(std::string_view a, std::string_view b) {
    return a.size() >= b.size() ? a : b;
}

struct Excerpt {
    std::string_view part;
};

int main() {
    std::string novel = "Call me Ishmael. Some years ago...";
    std::string_view first = std::string_view(novel).substr(0, novel.find('.'));
    Excerpt excerpt{first};
    std::string_view l = longest(excerpt.part, "short");
    std::cout << "longest: " << l << "\n";
    std::cout << "excerpt length: " << excerpt.part.size() << "\n";
}
