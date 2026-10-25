// Every C++ cell of the idiom table on the page, in the order of the table.
#include <iostream>
#include <memory>
#include <mutex>
#include <optional>
#include <ranges>
#include <string>
#include <string_view>
#include <thread>
#include <unordered_map>
#include <variant>
#include <vector>
#include <concepts>

struct Shape {
    virtual ~Shape() = default;
    virtual double area() const = 0;
};
struct Square : Shape {
    double side;
    explicit Square(double s) : side(s) {}
    double area() const override { return side * side; }
};

template <std::totally_ordered T>
T largest(T a, T b) { return a > b ? a : b; }

struct Guard {
    ~Guard() { std::cout << "15 bye\n"; }
};

int main() {
    {   // 1
        auto p = std::make_unique<int>(7);
        std::cout << "1 " << *p << "\n";
    }
    {   // 2
        auto s = std::make_shared<int>(7);
        auto t = s;
        std::cout << "2 " << *t << " owners=" << s.use_count() << "\n";
    }
    std::vector<int> v{1, 2, 3};
    {   // 3
        v.push_back(4);
        std::cout << "3 " << v.size() << "\n";
    }
    {   // 4
        try {
            (void)v.at(9);
        } catch (const std::out_of_range &) {
            std::cout << "4 out_of_range\n";
        }
    }
    {   // 5
        std::string s = "hi";
        s += " there";
        std::cout << "5 " << s << "\n";
    }
    {   // 6
        std::string_view s = "hi";
        std::cout << "6 " << s.size() << "\n";
    }
    {   // 7
        std::optional<int> o = 5;
        if (o) std::cout << "7 " << *o << "\n";
    }
    {   // 8
        std::vector<std::variant<int, std::string>> ws{5, std::string("five")};
        for (const auto &w : ws)
            std::visit([](const auto &x) { std::cout << "8 " << x << "\n"; }, w);
    }
    {   // 9
        try {
            std::stoi("abc");
        } catch (const std::invalid_argument &) {
            std::cout << "9 invalid_argument\n";
        }
    }
    {   // 10
        int sum = 0;
        for (const auto &x : v) sum += x;
        std::cout << "10 " << sum << "\n";
    }
    {   // 11
        auto even = [](int x) { return x % 2 == 0; };
        auto times_ten = [](int x) { return x * 10; };
        auto evens = std::views::transform(std::views::filter(v, even), times_ten);
        std::cout << "11";
        for (int x : evens) std::cout << " " << x;
        std::cout << "\n";
    }
    {   // 12
        std::unordered_map<std::string, int> m;
        m["a"]++;
        m["a"]++;
        std::cout << "12 " << m["a"] << "\n";
    }
    {   // 13
        std::unique_ptr<Shape> sh = std::make_unique<Square>(3);
        std::cout << "13 " << sh->area() << "\n";
    }
    std::cout << "14 " << largest(3, 5) << "\n";
    {   // 15 (the destructor)
        Guard g;
    }
    {   // 16
        std::mutex m;
        int n = 0;
        {
            std::lock_guard<std::mutex> g(m);
            n++;
        }
        std::cout << "16 " << n << "\n";
    }
    {   // 17
        int result = 0;
        std::thread t([&result] { result = 42; });
        t.join();
        std::cout << "17 " << result << "\n";
    }
    {   // 18
        double x = 3.9;
        std::cout << "18 " << static_cast<int>(x) << "\n";
    }
    {   // 19
        constexpr int limit = 100;
        std::cout << "19 " << limit << "\n";
    }
    return 0;
}
