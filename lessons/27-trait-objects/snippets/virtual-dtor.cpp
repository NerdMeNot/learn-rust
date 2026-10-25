#include <cstdio>
#include <memory>

struct A { // no virtual destructor
    virtual void f() {}
};
struct B : A {
    ~B() { std::puts("~B ran"); }
};

struct C { // virtual destructor
    virtual void f() {}
    virtual ~C() = default;
};
struct D : C {
    ~D() { std::puts("~D ran"); }
};

int main() {
    {
        std::unique_ptr<A> p = std::make_unique<B>();
        std::puts("leaving the scope of unique_ptr<A>");
    }
    {
        std::unique_ptr<C> p = std::make_unique<D>();
        std::puts("leaving the scope of unique_ptr<C>");
    }
}
