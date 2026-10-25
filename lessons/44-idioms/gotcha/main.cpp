#include <iostream>
#include <string>

struct Config {
    std::string name;
    unsigned short port;
    bool verbose;
};

int main() {
    Config base{"service", 8080, false};
    Config test = base; // copies every field, the string too
    test.port = 9000;
    std::cout << base.name << " " << base.port << " " << base.verbose << "\n";
    std::cout << test.name << " " << test.port << " " << test.verbose << "\n";
}
