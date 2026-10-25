#include <iostream>
#include <string>

// 1. A wrapper struct: a distinct type. `value` is public, so this is a label, not a guarantee.
struct UserId {
    explicit UserId(int v) : value(v) {}
    int value;
};

// 2. C++20 designated initializers: members left out keep their default (or are zero).
struct Config {
    std::string host = "localhost";
    int port = 8080;
    int retries = 3;
};

int main() {
    UserId id{7};
    std::cout << "UserId " << id.value << "\n";

    Config c{.port = 9000};
    std::cout << c.host << " " << c.port << " " << c.retries << "\n";
}
