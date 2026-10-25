#include <cstdio>
#include <vector>

struct Base {
    virtual const char* name() const { return "base"; }
};
struct Derived : Base {
    const char* name() const override { return "derived"; }
};

int main() {
    Derived d;
    Base copy = d; // copies only the Base part
    Base& ref = d;
    std::vector<Base> v;
    v.push_back(d); // so does the vector
    std::printf("copy: %s\nref: %s\nvector item: %s\n", copy.name(), ref.name(), v[0].name());
}
