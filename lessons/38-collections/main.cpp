#include <algorithm>
#include <deque>
#include <functional>
#include <iostream>
#include <map>
#include <queue>
#include <ranges>
#include <string_view>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

int main() {
    std::string_view text = "the cat saw the dog and the cat ran";
    auto words = text | std::views::split(' ')
               | std::views::transform([](auto w) { return std::string_view(w.begin(), w.end()); });

    // unordered_map: count words. Iteration order is unspecified, so sort before printing.
    std::unordered_map<std::string_view, int> counts;
    for (auto w : words) counts[w] += 1; // [] inserts 0 for a new word
    std::vector<std::pair<std::string_view, int>> pairs(counts.begin(), counts.end());
    std::ranges::sort(pairs, [](const auto &a, const auto &b) {
        return a.second != b.second ? a.second > b.second : a.first < b.first;
    });
    std::cout << "top:";
    for (auto &[w, n] : pairs | std::views::take(3)) std::cout << " " << w << "=" << n;
    std::cout << "\n";

    // std::map: keys always come out sorted
    std::map<std::string_view, int> by_name{{"zoe", 31}, {"ana", 30}, {"mia", 28}};
    std::cout << "sorted:";
    for (auto &[name, age] : by_name) std::cout << " " << name << "=" << age;
    std::cout << "\n";

    // unordered_set: unique items, fast "is it in there?"
    std::unordered_set<std::string_view> seen(words.begin(), words.end());
    std::cout << seen.size() << " unique words, has dog: " << std::boolalpha << seen.contains("dog") << "\n";

    // deque: push at the back, pop at the front
    std::deque<std::string_view> queue{"a", "b"};
    queue.push_back("c");
    auto first = queue.front();
    queue.pop_front();
    std::cout << "served " << first << ", waiting " << queue[0] << " " << queue[1] << "\n";

    // priority_queue pops the largest first. std::greater flips it to the smallest.
    std::priority_queue<int, std::vector<int>, std::greater<int>> heap;
    for (int n : {5, 1, 8, 3}) heap.push(n);
    std::cout << "smallest first:";
    while (!heap.empty()) {
        std::cout << " " << heap.top();
        heap.pop();
    }
    std::cout << "\n";
}
