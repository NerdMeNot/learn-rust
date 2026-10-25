#include <iostream>
#include <string>

int main() {
    const std::string name = "gopher";  // const: cannot change
    int count = 1;                      // plain variables can change
    count += 1;

    const int total = count * 10;       // no shadowing in the same scope: pick a new name
    std::cout << name << " " << total << "\n";
}
