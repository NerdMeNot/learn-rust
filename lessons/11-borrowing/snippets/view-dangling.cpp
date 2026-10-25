#include <iostream>
#include <string>
#include <string_view>

int main() {
    std::string_view view;
    {
        std::string name = "Ada";
        view = name; // a view of name's characters
    }
    std::cout << "view: " << view << "\n"; // name was destroyed above
}
