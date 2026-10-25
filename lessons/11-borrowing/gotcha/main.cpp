#include <iostream>
#include <vector>

int total(std::vector<int> xs) {
    int sum = 0;
    for (int x : xs) {
        sum += x;
    }
    return sum;
}

int main() {
    std::vector<int> nums{1, 2, 3};
    int t = total(nums);
    std::cout << t << " [" << nums[0] << ", " << nums[1] << ", " << nums[2] << "]\n";
}
