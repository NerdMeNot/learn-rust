#include <iostream>

int main(int argc, char **) {
    int *p = new int(42);
    if (argc > 1) {
        return 1;       // leaves early: the delete below never runs
    }
    std::cout << *p << "\n";
    delete p;
    return 0;
}
