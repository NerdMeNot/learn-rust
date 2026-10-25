#include <iostream>
#include <optional>
#include <vector>

// What other languages write with ?:, i++, null and switch, written in C++.
const char *size(int n) {
    switch (n) {
    case 1:
    case 2:
        return "small";
    case 3:
        return "medium";
    default:
        return "large";
    }
}

std::optional<int> first_even(const std::vector<int> &items) {
    for (int x : items) {
        if (x % 2 == 0) {
            return x;
        }
    }
    return std::nullopt;
}

int main() {
    int total = 0;
    for (int i = 0; i < 4; i++) {
        total += i;
    }
    const char *label = total > 5 ? "big" : "small";
    std::cout << "total: " << total << " (" << label << ")\n";
    std::cout << "size: " << size(2) << " " << size(9) << "\n";
    auto even = first_even({3, 5, 8});
    if (even) {
        std::cout << "first even: " << *even << "\n";
    } else {
        std::cout << "no even number\n";
    }
    std::cout << "none: " << first_even({1, 3}).value_or(-1) << "\n";
    long long wide = static_cast<long long>(total) * 1000000000LL;
    std::cout << "wide: " << wide << "\n";
}
