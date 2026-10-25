#include <iostream>
#include <string_view>

int add(int a, int b) {
    return a + b;
}

void greet(std::string_view name) {   // void: returns nothing
    std::cout << "hi " << name << "\n";
}

int main() {
    std::cout << add(2, 3) << "\n";
    greet("kotlin");
}
