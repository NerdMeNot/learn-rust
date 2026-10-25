#include <iostream>
#include <string>

struct User {
    std::string name;
    unsigned age;
    bool active;
};

// C++ has no derive: you write the printing yourself
std::ostream& operator<<(std::ostream& os, const User& u) {
    return os << "User { name: \"" << u.name << "\", age: " << u.age
              << ", active: " << std::boolalpha << u.active << " }";
}

int main() {
    User u{.name = "ana", .age = 30, .active = true};
    std::cout << u.name << " is " << u.age << ", active=" << std::boolalpha << u.active << "\n";
    std::cout << u << "\n";
}
