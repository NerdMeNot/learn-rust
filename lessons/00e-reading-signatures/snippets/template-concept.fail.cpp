#include <concepts>
#include <iostream>
#include <vector>

template <std::totally_ordered T>
const T& largest(const std::vector<T>& list) {
    const T* best = &list[0];
    for (const T& item : list) {
        if (item > *best) {
            best = &item;
        }
    }
    return *best;
}

struct Point {
    int x;
    int y;
};

int main() {
    std::vector<Point> points{{1, 2}, {3, 4}};
    const Point& best = largest(points);
    std::cout << "x: " << best.x << "\n";
}
