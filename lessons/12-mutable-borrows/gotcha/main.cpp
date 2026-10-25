#include <iostream>
#include <vector>

int main() {
    std::vector<int> scores{10, 20, 30};
    int& first = scores[0];
    int& last = scores[2];
    first += 1;
    last += 1;
    std::cout << "[" << scores[0] << ", " << scores[1] << ", " << scores[2] << "]\n";
}
