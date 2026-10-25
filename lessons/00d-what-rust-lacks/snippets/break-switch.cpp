#include <iostream>

int main() {
    int n = 0;
    for (int i = 0; i < 5; i++) {
        switch (i) {
        case 2:
            break;
        default:
            n++;
        }
    }
    std::cout << "n: " << n << "\n";
}
