#include <cassert>
#include <concepts>
#include <fstream>
#include <functional>
#include <iostream>
#include <optional>
#include <stdexcept>
#include <string>
#include <vector>

template <std::totally_ordered T>
const T& largest(const std::vector<T>& list) {
    const T* best = &list[0];
    for (const T& item : list) {
        if (item > *best) best = &item;
    }
    return *best;
}

int apply(std::function<int(int)> f, int x) { return f(x); }

std::optional<int> find(const std::vector<int>& a, int key) {
    for (int n : a) {
        if (n == key) return n;
    }
    return std::nullopt;
}

std::string read(const std::string& path) {
    std::ifstream in(path);
    if (!in) throw std::runtime_error("cannot open " + path);
    return std::string(std::istreambuf_iterator<char>(in), {});
}

class Log {
public:
    void add(const std::string& line) { lines.push_back(line); }
    std::size_t count() const { return lines.size(); }

private:
    std::vector<std::string> lines;
};

void bump(int& x) { x++; }

// a concept is met structurally: this struct never says it is "ordered"
struct Plain {
    int n;
    auto operator<=>(const Plain&) const = default;
};
static_assert(std::totally_ordered<Plain>);
static_assert(std::totally_ordered<double>);

int main() {
    assert(largest(std::vector<int>{3, 9, 4}) == 9);
    assert(apply([](int x) { return x + 1; }, 1) == 2);
    assert(!find({1, 2}, 5).has_value());
    bool threw = false;
    try {
        read("/nope");
    } catch (const std::runtime_error&) {
        threw = true;  // nothing in the signature said this could throw
    }
    assert(threw);
    Log log;
    log.add("hello");
    assert(log.count() == 1);
    int n = 1;
    bump(n);
    assert(n == 2);
    std::cout << "cpp rows ok\n";
}
