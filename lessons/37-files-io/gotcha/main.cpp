#include <filesystem>
#include <iostream>
#include <string>

int main() {
    auto path = std::filesystem::temp_directory_path() / "rbc-hello.txt";
    std::ofstream(path) << "hello";

    std::ifstream f(path);
    std::string text;
    std::getline(f, text);
    std::cout << text << "\n";
}
