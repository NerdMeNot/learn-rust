#include <cstdlib>
#include <iostream>
#include <string>

int main() {
    std::string port = std::getenv("RBC_PORT");
    std::cout << "listening on port " << port << "\n";
}
