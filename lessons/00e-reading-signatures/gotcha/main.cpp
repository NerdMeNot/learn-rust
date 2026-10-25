#include <iostream>
#include <string>
#include <vector>

template <typename T>
const T& largest(const std::vector<T>& list) {
    const T* best = &list[0];
    for (const T& item : list) {
        if (item > *best) {
            best = &item;
        }
    }
    return *best;
}

int main() {
    std::vector<int> numbers{3, 9, 4};
    std::vector<std::string> words{"pear", "fig", "apple"};
    std::cout << "largest number: " << largest(numbers) << "\n";
    std::cout << "largest word: " << largest(words) << "\n";
}
