#include <iostream>
#include <memory>
#include <string>
#include <string_view>

int main() {
    auto name = std::make_shared<std::string>("a name that does not fit inline");
    std::string_view view = *name;
    name.reset(); // the last owner lets go: the string is freed
    std::cout << "view: " << view << "\n";
}
