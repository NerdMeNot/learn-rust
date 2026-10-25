#include <algorithm>
#include <iostream>
#include <vector>

int main() {
    // Room to spare: push_back does not reallocate, yet the loop cached end() before it started.
    std::vector<int> v{1, 2, 3, 4, 5, 6};
    v.reserve(100);
    int passes = 0;
    for (int x : v) {
        passes++;
        if (x == 2) v.push_back(99);
    }
    std::cout << "push_back: " << passes << " passes, size " << v.size() << "\n";

    // erase in a range-for: no sanitizer report either.
    std::vector<int> w{1, 2, 3, 4, 5, 6};
    for (int x : w) {
        if (x % 2 == 0) w.erase(std::find(w.begin(), w.end(), x));
    }
    std::cout << "erase evens, left:";
    for (int x : w) std::cout << " " << x;
    std::cout << "\n";
}
