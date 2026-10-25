#include <iostream>
#include <string_view>

std::string_view kind_of(int day) {
    switch (day) {
        case 1: case 2: case 3: case 4: case 5:
            return "weekday";
        case 6: case 7:
            return "weekend";
        default:                // anything else
            return "not a day";
    }
}

int main() {
    int day = 6;
    std::cout << kind_of(day) << "\n";
}
