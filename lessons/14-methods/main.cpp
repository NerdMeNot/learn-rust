#include <iostream>

class Counter {
public:
    unsigned get() const { return count_; }   // const: only reads
    void bump() { count_ += 1; }               // not const: changes the object

private:
    unsigned count_ = 0;                       // a default member initializer: starts at 0
};

int main() {
    Counter c;
    c.bump();
    c.bump();
    std::cout << c.get() << "\n";
}
