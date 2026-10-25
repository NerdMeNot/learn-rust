#include <iostream>
#include <string>

struct User {
    std::string name;
    unsigned age;
    bool active;
};

int main() {
    User u{.name = "ana"};
    std::cout << u.name << " " << u.age << " " << std::boolalpha << u.active << "\n";
}
