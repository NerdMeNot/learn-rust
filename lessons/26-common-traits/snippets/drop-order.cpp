#include <iostream>
#include <string>

struct Noisy {
    std::string name;
    explicit Noisy(std::string n) : name(std::move(n)) {}
    ~Noisy() { std::cout << "  drop " << name << "\n"; }
};

struct Pair {
    Noisy first{"first"};
    Noisy second{"second"};
};

int main() {
    std::cout << "locals:\n";
    {
        Noisy a("a");
        Noisy b("b");
        Noisy c("c");
        std::cout << "  end of block\n";
    }
    std::cout << "members:\n";
    {
        Pair p;
        std::cout << "  end of block\n";
    }
}
