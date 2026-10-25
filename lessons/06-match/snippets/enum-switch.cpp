#include <iostream>

enum Light { RED, YELLOW, GREEN };

int main() {
    Light light = YELLOW;
    const char *action = "no action";
    switch (light) {
    case RED:
        action = "stop";
        break;
    case GREEN:
        action = "go";
        break;
    }
    std::cout << action << "\n";
}
