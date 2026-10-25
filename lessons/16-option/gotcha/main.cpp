#include <iostream>
#include <optional>
#include <string>

struct User {
    std::string name;
};

std::optional<User> find_user(int id) {
    if (id == 1) return User{"ana"};
    return std::nullopt;
}

int main() {
    auto u = find_user(2);
    std::cout << u.name << "\n";
}
