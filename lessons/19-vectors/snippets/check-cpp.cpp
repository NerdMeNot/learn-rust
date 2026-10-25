#include <algorithm>
#include <array>
#include <cassert>
#include <iostream>
#include <span>
#include <vector>

int main() {
    std::vector<int> v{3, 1, 2};
    v.push_back(4);
    v.pop_back();
    assert(v.back() == 2 && v.front() == 3 && v.size() == 3 && !v.empty());
    v.insert(v.begin() + 1, 5);
    v.erase(v.begin());
    assert((v == std::vector<int>{5, 1, 2}));

    v.erase(std::remove_if(v.begin(), v.end(), [](int x) { return x == 1; }), v.end());
    assert((v == std::vector<int>{5, 2}));

    std::sort(v.begin(), v.end());
    assert(std::binary_search(v.begin(), v.end(), 5));

    std::vector<int> runs{1, 1, 2, 2, 1};
    runs.erase(std::unique(runs.begin(), runs.end()), runs.end());
    assert((runs == std::vector<int>{1, 2, 1}));

    v.reserve(100);
    assert(v.capacity() >= 100);

    std::array<int, 4> a{1, 2, 3, 4};
    std::span<const int> s = std::span(a).subspan(1, 2);
    assert(s.size() == 2 && s[0] == 2);
    std::vector<int> filled(3, 0);
    filled.resize(5, 7);
    assert((filled == std::vector<int>{0, 0, 0, 7, 7}));
    filled.erase(filled.begin() + 1, filled.begin() + 3);
    assert((filled == std::vector<int>{0, 7, 7}));
    std::cout << "all checks passed\n";
}
