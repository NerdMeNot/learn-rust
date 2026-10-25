#include <array>
#include <iostream>
#include <string>

struct Point {
    int x, y;
};

int main() {
    // structured binding: by position
    auto [x, y] = Point{3, -2};
    std::cout << "x = " << x << ", y = " << y << "\n";

    std::string abc = "abc";
    for (size_t i = 0; i < abc.size(); i++) {
        std::cout << i << ": " << abc[i] << "\n";
    }

    std::string line = "mode=fast";
    if (auto eq = line.find('='); eq != std::string::npos) {
        std::cout << line.substr(0, eq) << " -> " << line.substr(eq + 1) << "\n";
    }

    // an array is read by index
    std::array<int, 4> a = {10, 20, 30, 40};
    std::cout << "first = " << a.front() << ", last = " << a.back() << "\n";
}
