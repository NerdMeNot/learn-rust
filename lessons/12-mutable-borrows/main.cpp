#include <iostream>
#include <vector>

void add_bonus(std::vector<int>& scores, int bonus) {   // non-const reference: may change it
    for (int& s : scores) {
        s += bonus;
    }
    scores.push_back(0);
}

int main() {
    std::vector<int> scores{10, 20};
    add_bonus(scores, 5);
    std::cout << "[";
    for (std::size_t i = 0; i < scores.size(); ++i) {
        std::cout << (i ? ", " : "") << scores[i];
    }
    std::cout << "]\n";
}
