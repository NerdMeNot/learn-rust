#include <iostream>
#include <vector>

template <typename T>
const T& largest(const std::vector<T>& items) {
    std::cout << __PRETTY_FUNCTION__ << "\n";
    const T* best = &items[0];
    for (const T& item : items) {
        if (item > *best) {
            best = &item;
        }
    }
    return *best;
}

int main() {
    largest(std::vector<int>{3, 9, 2});
    largest(std::vector<double>{1.5, 0.5});
}
