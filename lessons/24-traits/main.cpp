#include <algorithm>
#include <cctype>
#include <format>
#include <iostream>
#include <string>

class Speaker {
public:
    virtual ~Speaker() = default;
    virtual std::string speak() const = 0;

    std::string shout() const {                 // shared method built on speak()
        std::string s = speak();
        std::ranges::transform(s, s.begin(), [](unsigned char c) { return std::toupper(c); });
        return s;
    }
};

class Dog : public Speaker {
public:
    std::string speak() const override { return "woof"; }
};

class Robot : public Speaker {
public:
    explicit Robot(unsigned id) : id_(id) {}
    std::string speak() const override { return std::format("beep {}", id_); }

private:
    unsigned id_;
};

void introduce(const Speaker& s) {   // any class that derives from Speaker
    std::cout << s.speak() << " / " << s.shout() << "\n";
}

int main() {
    introduce(Dog{});
    introduce(Robot{7});
}
