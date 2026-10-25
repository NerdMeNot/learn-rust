#include <concepts>
#include <iostream>
#include <string_view>
#include <vector>

// T can be any type that can be compared with < and >
template <std::totally_ordered T>
const T& largest(const std::vector<T>& items) {
    const T* best = &items[0];
    for (const T& item : items) {
        if (item > *best) {
            best = &item;
        }
    }
    return *best;
}

int main() {
    std::vector<int> nums{3, 9, 2};
    std::vector<std::string_view> fruit{"kiwi", "apple", "pear"};
    std::cout << largest(nums) << "\n";
    std::cout << largest(fruit) << "\n";
}
