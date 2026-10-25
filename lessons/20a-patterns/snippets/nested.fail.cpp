#include <utility>

int main() {
    std::pair<int, std::pair<int, int>> p{1, {2, 3}};
    auto [a, [b, c]] = p;
    return a;
}
