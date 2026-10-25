#include <cstdlib>
#include <iostream>
#include <optional>
#include <stdexcept>
#include <string>

int visits = 0;

static void check(bool ok, const char *what) {
    if (!ok) {
        std::cerr << "failed: " << what << "\n";
        std::exit(1);
    }
}

class Animal {
public:
    explicit Animal(std::string n) : name(std::move(n)) {}
    virtual ~Animal() = default;
    virtual std::string speak() const { return name + " makes a sound"; }
    std::string name;
};

class Dog : public Animal {
public:
    explicit Dog(std::string n) : Animal(std::move(n)) {}
    std::string speak() const override { return name + " barks"; }
};

struct P {
    int x;
    P(int v) : x(v) {}
    P operator+(const P &o) const { return P(x + o.x); }
};

struct Guard {
    std::string &log;
    explicit Guard(std::string &l) : log(l) {}
    ~Guard() { log += "destroyed"; }
};

static int area(int width, int height = 1) { return width * height; }
static std::string show(int n) { return "int " + std::to_string(n); }
static std::string show(const std::string &s) { return "text " + s; }

template <typename T>
static T largest(T a, T b) { return a > b ? a : b; }

static int parse(const std::string &s) { return std::stoi(s) + 1; }

int main() {
    // ++ and --, also inside expressions, and --x decrements
    int a[] = {10, 20, 30};
    int i = 0;
    int first = a[i++];
    int second = a[++i];
    check(first == 10 && second == 30 && i == 2, "inc");
    int j = 5;
    int pre = --j;
    check(pre == 4 && j == 4, "--x decrements");

    // the ternary operator, and auto
    const char *label = i > 1 ? "big" : "small";
    auto inferred = 5;
    check(label[0] == 'b' && inferred == 5, "ternary");

    // for (;;), do-while
    int spins = 0;
    for (;;) {
        spins++;
        break;
    }
    do {
        spins++;
    } while (false);
    check(spins == 2, "loops");

    // nullptr, a pointer used as a condition, and std::optional
    const int *p = nullptr;
    std::optional<int> none;
    check(p == nullptr && !p && !none.has_value(), "null");

    // implicit conversions
    int n = 3.9;
    long l = i;
    double d = 3;
    check(n == 3 && l == 2 && d == 3.0, "convert");

    // new and delete, a constructor and a destructor
    P *heap = new P(1);
    check(heap->x == 1, "new");
    delete heap;
    std::string log;
    {
        Guard g(log);
        log += "body,";
    }
    check(log == "body,destroyed", "destructor");

    // overloads, default arguments, operators and templates
    check(area(3) == 3 && area(3, 4) == 12, "default");
    check(show(1) == "int 1" && show(std::string("a")) == "text a", "overload");
    check((P(1) + P(2)).x == 3, "operator");
    check(largest(2, 3) == 3, "template");

    // inheritance and virtual functions
    Dog dog("Rex");
    const Animal &animal = dog;
    check(animal.speak() == "Rex barks", "virtual");

    // switch falls through
    int out = 0;
    switch (1) {
    case 1:
        out += 1;
        [[fallthrough]];
    case 2:
        out += 10;
        break;
    default:
        out += 100;
    }
    check(out == 11, "fallthrough");

    // exceptions
    bool caught = false;
    try {
        parse("x");
    } catch (const std::invalid_argument &) {
        caught = true;
    }
    check(caught && parse("41") == 42, "catch");

    // goto
    int k = 0;
again:
    k++;
    if (k < 3) {
        goto again;
    }
    check(k == 3, "goto");

    visits++;
    check(visits == 1, "global");

    std::cout << "cpp: all checks passed\n";
    return 0;
}
