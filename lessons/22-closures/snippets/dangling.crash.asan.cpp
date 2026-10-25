#include <functional>
#include <iostream>

std::function<int(int)> make_adder(int n) {
    return [&](int x) { return x + n; };   // n dies when make_adder returns
}

int main() {
    auto add5 = make_adder(5);
    std::cout << add5(1) << "\n";
}
