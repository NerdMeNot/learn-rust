#include <iostream>

static int log_msg(const char *msg) {
    std::cout << "log: " << msg << "\n";
    return 1;
}

int main() {
    int n = 7;

    const char *size = n > 5 ? "big" : "small";
    std::cout << "ternary: " << size << "\n";

    int area = [] {
        int w = 3;
        int h = 4;
        return w * h;
    }();
    std::cout << "block: " << area << "\n";

    const char *name;
    switch (2) {
    case 1:
        name = "one";
        break;
    case 2:
        name = "two";
        break;
    default:
        name = "many";
    }
    std::cout << "switch: " << name << "\n";

    int a = 1, b = 2;
    std::swap(a, b);
    std::cout << "swap: " << a << " " << b << "\n";

    int c = (log_msg("first"), 2);
    std::cout << "comma: " << c << "\n";

    auto addOne = [](int x) {
        int y = x + 1;
        return y;
    };
    std::cout << "closure: " << addOne(41) << "\n";
}
