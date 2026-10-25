#include <iostream>

int clamp(int x) {
    if (x < 0) {
        return 0;
    }
    return x > 100 ? 100 : x;
}

int main() {
    int n = 7;
    const char *label = n % 2 == 0 ? "even" : "odd";
    int area = [] {
        int w = 3;
        int h = 4;
        return w * h;
    }();
    std::cout << n << " is " << label << "\n";
    std::cout << "area: " << area << "\n";
    std::cout << "clamp: " << clamp(-5) << " " << clamp(50) << " " << clamp(250) << "\n";
}
