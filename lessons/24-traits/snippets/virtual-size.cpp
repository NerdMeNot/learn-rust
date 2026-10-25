#include <iostream>

struct PlainDog {};

struct Speaker {
    virtual ~Speaker() = default;
    virtual const char* speak() const = 0;
};

struct Dog : Speaker {
    const char* speak() const override { return "woof"; }
};

int main() {
    std::cout << "sizeof(PlainDog): " << sizeof(PlainDog) << "\n";
    std::cout << "sizeof(Dog): " << sizeof(Dog) << "\n";
    std::cout << Dog{}.speak() << "\n";
}
