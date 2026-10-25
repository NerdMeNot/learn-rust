#include <format>
#include <iostream>
#include <iterator>
#include <string>

int main() {
    std::string s = std::format("n = {}", 42);
    std::cout << s << ", capacity " << s.capacity() << "\n";

    std::string out;
    std::format_to(std::back_inserter(out), "{}-{}", 1, 2);  // write! into a String
    std::cout << out << "\n";

    int n = 7;
    std::string_view word = "x";
    std::cout << std::vformat("{} {}", std::make_format_args(n, word)) << "\n";  // format_args!
}
