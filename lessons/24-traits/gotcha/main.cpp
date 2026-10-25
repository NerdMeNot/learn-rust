#include <format>
#include <iostream>
#include <string>

class Speaker {
public:
    virtual ~Speaker() = default;
    virtual std::string speak() const = 0;
};

class Robot {
public:
    unsigned id;
    std::string speak() const { return std::format("beep {}", id); }
};

void introduce(const Speaker& s) {
    std::cout << s.speak() << "\n";
}

int main() {
    introduce(Robot{7});
}
