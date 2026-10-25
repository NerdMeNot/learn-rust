#include <cassert>
#include <cmath>
#include <compare>
#include <type_traits>
#include <iostream>
#include <stdexcept>
#include <string>

struct Version {
    unsigned major;
    unsigned minor;
    auto operator<=>(const Version &) const = default;
};

struct Whole {
    int a;
    auto operator<=>(const Whole &) const = default;
};
struct Fract {
    double a;
    auto operator<=>(const Fract &) const = default;
};
// the defaulted <=> is strong for integer members and partial once a double is a member
static_assert(std::is_same_v<decltype(Whole{} <=> Whole{}), std::strong_ordering>);
static_assert(std::is_same_v<decltype(Fract{} <=> Fract{}), std::partial_ordering>);

// a converting constructor that is not explicit converts silently
struct Meters {
    double v;
    Meters(double v) : v(v) {}
};
double twice(Meters m) { return m.v * 2; }

int main() {
    assert(twice(2.5) == 5.0);
    assert(!(Fract{NAN} < Fract{1}) && !(Fract{NAN} > Fract{1}));
    assert(std::stoi("4x") == 4);
    bool threw = false;
    try {
        std::stoi("x");
    } catch (const std::invalid_argument &) {
        threw = true;
    }
    assert(threw);
    // a defaulted <=> compares the members in the order they are declared
    assert((Version{1, 10} > Version{1, 9}));
    assert((Version{0, 99} < Version{1, 0}));
    std::cout << "all checks passed\n";
}
