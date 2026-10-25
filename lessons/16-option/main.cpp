#include <iostream>
#include <optional>
#include <string>

struct User {
    std::string name;
};

std::optional<User> find_user(int id) {
    if (id == 1) {
        return User{"ana"};
    }
    return std::nullopt;
}

int main() {
    if (auto u = find_user(1)) {     // true only when it holds a value
        std::cout << "found " << u->name << "\n";
    } else {
        std::cout << "no user\n";
    }

    if (auto u = find_user(2)) {
        std::cout << "found " << u->name << "\n";
    } else {
        std::cout << "no user\n";
    }
}
