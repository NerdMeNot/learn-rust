#include <iostream>
#include <ranges>
#include <string>
#include <string_view>

std::string make() { return "alpha,beta,gamma, long enough to need the heap"; }

// The first piece of the text, as a view into the caller's string.
std::string_view first_piece(const std::string& text) {
    for (auto part : std::string_view(text) | std::views::split(',')) {
        return std::string_view(part.begin(), part.end());
    }
    return {};
}

int main() {
    // make() returns a temporary string; it is destroyed at the semicolon.
    std::string_view piece = first_piece(make());
    std::cout << "piece.size(): " << piece.size() << "\n";
}
