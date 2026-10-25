#include <iostream>
#include <memory>

struct Counter {
    int hits = 0;
};

int main() {
    auto a = std::make_shared<Counter>();
    auto b = a;

    a->hits++;
    b->hits++;
    std::cout << "hits: " << a->hits << "\n";
}
