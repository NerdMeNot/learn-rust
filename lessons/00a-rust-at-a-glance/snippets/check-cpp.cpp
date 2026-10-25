#include <cassert>
#include <iostream>
#include <string>
#include <vector>

int add(int a, int b) { return a + b; }

int main() {
    int x = 5;
    x = 6;
    const int y = 5;
    assert(x == 6 && y == 5);
    assert(add(2, 3) == 5);

    int count = 0;
    for (int i = 0; i < 3; i++) {
        count += i;
    }
    assert(count == 3);

    std::vector<std::string> words{"a", "b"};
    std::string joined;
    for (const auto& w : words) {
        joined += w;
    }
    assert(joined == "ab");

    int n = 1;
    std::string s;
    switch (n) {
    case 1:
        s = "one";
        break;
    default:
        s = "many";
    }
    assert(s == "one");

    auto inc = [](int v) { return v + 1; };
    assert(inc(1) == 2);

    std::string name = "Ana";
    int age = 30;
    std::cout << name << " is " << age << "\n";

    bool threw = false;
    try {
        std::stoi("x");
    } catch (const std::invalid_argument&) {
        threw = true;
    }
    assert(threw);
    std::cout << "all c++ cells compile and hold\n";
}
