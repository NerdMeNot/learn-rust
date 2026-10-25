#include <iostream>
#include <string>
#include <utility>
#include <vector>

struct Person {
    std::string name;
    int age;
};

struct Counter {
    int n = 0;
    void inc() { n++; } // `this` points at the object the method was called on
};

static void bump_copy(Counter c) { c.inc(); }
static void bump_ref(Counter &c) { c.inc(); }

int main() {
    std::vector<Person> people{{"Ada", 36}, {"Linus", 54}};

    for (auto p : people) { // `auto` copies each element
        p.age++;
    }
    std::cout << "after for (auto p : people):  " << people[0].age << ", " << people[1].age << "\n";

    for (auto &p : people) { // `auto&` is the element itself
        p.age++;
    }
    std::cout << "after for (auto& p : people): " << people[0].age << ", " << people[1].age << "\n";

    // Lambda captures: [=] copies the variable, [&] refers to it
    int count = 0;
    auto by_copy = [=]() mutable { return ++count; };
    auto by_ref = [&count]() { return ++count; };
    by_copy();
    by_copy();
    by_ref();
    std::cout << "count after two by_copy calls and one by_ref call: " << count << "\n";

    Counter c;
    c.inc();
    std::cout << "c.n after c.inc():        " << c.n << "\n";
    bump_copy(c);
    std::cout << "c.n after bump_copy(c):   " << c.n << "\n";
    bump_ref(c);
    std::cout << "c.n after bump_ref(c):    " << c.n << "\n";
    Counter d = c; // the copy constructor copies every member
    d.inc();
    std::cout << "c.n after d = c; d.inc(): " << c.n << "\n";

    // std::move on an object leaves it valid, but its contents are unspecified
    std::string s = "hello";
    std::string t = std::move(s);
    std::cout << "after t = std::move(s): t = " << t << ", s.size() = " << s.size() << "\n";
    int x = 5;
    int y = std::move(x); // an int has nothing to steal, so this is a plain copy
    std::cout << "after y = std::move(x): y = " << y << ", x = " << x << "\n";
    return 0;
}
