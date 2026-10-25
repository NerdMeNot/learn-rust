#include <cstdlib>
#include <iostream>

struct Guard {
    const char *name;
    ~Guard() { std::cout << "  ~Guard " << name << " ran\n"; }
};

static Guard global_guard{"global"};

int main() {
    Guard local{"local"};
    std::cout << "calling std::exit\n";
    std::exit(0);
}
