#include <iostream>
#include <string>

int main() {
    int day = 99;
    std::string kind;
    switch (day) {
        case 1: case 2: case 3: case 4: case 5:
            kind = "weekday";
            break;
        case 6: case 7:
            kind = "weekend";
            break;
    }
    std::cout << kind << "\n";
}
