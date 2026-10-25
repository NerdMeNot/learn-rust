#include <iostream>
#include <span>
#include <vector>

int total(std::span<const int> xs) {   // a read-only view: borrow, don't copy
    int sum = 0;
    for (int x : xs) {
        sum += x;
    }
    return sum;
}

int main() {
    std::vector<int> nums{1, 2, 3};
    int t = total(nums);                // lend nums to total
    std::cout << t << " [";             // nums is still ours
    for (std::size_t i = 0; i < nums.size(); ++i) {
        std::cout << (i ? ", " : "") << nums[i];
    }
    std::cout << "]\n";
}
