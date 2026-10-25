#include <iostream>
#include <vector>

int main() {
    std::vector<int> v{1, 2, 3, 4};    // capacity 4: full
    for (int x : v) {                   // the loop walks the buffer it saw first
        if (x == 2) v.push_back(99);    // the push moves the elements to a new buffer
        std::cout << "saw " << x << "\n";
    }
}
