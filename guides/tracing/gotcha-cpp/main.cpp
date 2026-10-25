#include <iostream>
#include <string>

struct Span {
    std::string name;
    explicit Span(std::string n) : name(std::move(n)) { std::cout << "enter " << name << "\n"; }
    ~Span() { std::cout << "exit " << name << "\n"; }
};

int main() {
    {
        Span span("request");  // a named object lives to the end of the block
        std::cout << "listed notes\n";
    }
    {
        Span("temporary");  // no name: destroyed at the end of this statement
        std::cout << "listed notes\n";
    }
}
