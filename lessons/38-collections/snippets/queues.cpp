#include <algorithm>
#include <deque>
#include <iostream>
#include <iterator>
#include <map>
#include <queue>
#include <set>
#include <vector>

int main() {
    std::priority_queue<int> big;  // a max-heap by default, like BinaryHeap
    for (int n : {3, 9, 4, 1}) big.push(n);
    std::cout << "priority_queue top: " << big.top() << "\n";
    std::priority_queue<int, std::vector<int>, std::greater<int>> small;
    for (int n : {3, 9, 4, 1}) small.push(n);
    std::cout << "with greater<int>, top: " << small.top() << "\n";

    std::map<int, const char*> log{{10, "boot"}, {25, "login"}, {40, "upload"}};
    auto floor = std::prev(log.upper_bound(30));  // last key <= 30
    std::cout << "floor(30): " << floor->first << ", lower_bound(30): " << log.lower_bound(30)->first << "\n";

    auto held = log.find(10);  // an iterator into the tree
    for (int k = 11; k < 200; ++k) log[k] = "x";
    log.erase(25);
    std::cout << "after 189 inserts and an erase, the iterator to key 10 still reads " << held->second << "\n";

    std::set<int> a{1, 2, 3, 4}, b{3, 4, 5};
    std::vector<int> both;
    std::set_intersection(a.begin(), a.end(), b.begin(), b.end(), std::back_inserter(both));
    std::cout << "set_intersection size: " << both.size() << "\n";

    std::deque<int> dq{2, 3};
    dq.push_front(1);
    std::cout << "deque[0]: " << dq[0] << ", back: " << dq.back() << "\n";
    int &front = dq.front();
    for (int i = 0; i < 1000; ++i) { dq.push_back(i); dq.push_front(-i); }
    std::cout << "after 2000 pushes the reference to the old front reads " << front << "\n";
}
