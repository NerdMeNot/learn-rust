#include <iostream>
#include <string>
#include <vector>

int main() {
    std::vector<std::string> names{"ana", "bo"};

    for (const auto& n : names) {     // const reference: no copy, no change
        std::cout << "hi " << n << "\n";
    }

    std::vector<int> nums{1, 2, 3};
    for (auto& n : nums) {            // reference: change each item in place
        n *= 10;
    }

    for (std::size_t i = 0; i < nums.size(); ++i) {   // with an index
        std::cout << i << ": " << nums[i] << "\n";
    }
    std::cout << names.size() << " names, [";
    for (std::size_t i = 0; i < nums.size(); ++i) {
        std::cout << (i ? ", " : "") << nums[i];
    }
    std::cout << "]\n";
}
