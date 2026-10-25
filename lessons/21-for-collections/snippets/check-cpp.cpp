// Backs the C++ cells of the tables in lesson.mdx. Never shown.
#include <algorithm>
#include <cassert>
#include <iostream>
#include <ranges>
#include <string>
#include <vector>

int main() {
    std::vector<std::string> v{"ana", "bo", "cy"};
    std::string seen;
    for (const auto& x : v | std::views::reverse) {
        seen += x;
    }
    assert(seen == "cyboana");
    seen.clear();
    for (std::size_t i = 0; i < v.size(); ++i) {
        seen += v[i];
    }
    assert(seen == "anabocy");
    std::vector<int> e{1, 2, 3, 4, 5, 6};
    std::erase_if(e, [](int x) { return x % 2 == 0; });
    assert((e == std::vector<int>{1, 3, 5}));
    std::vector<int> r{1, 2, 3, 4, 5, 6};
    r.erase(std::remove_if(r.begin(), r.end(), [](int x) { return x % 2 == 0; }), r.end());
    assert((r == std::vector<int>{1, 3, 5}));
    std::cout << "ok\n";
}
