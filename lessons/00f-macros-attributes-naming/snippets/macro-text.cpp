#include <iostream>

#define SQUARE(x) x * x
#define DOUBLE_INTO(dst, src) \
    {                         \
        int tmp = (src) * 2;  \
        dst = tmp;            \
    }

int main() {
    std::cout << "SQUARE(1 + 2) = " << SQUARE(1 + 2) << "\n";
    int tmp = 0;
    std::cout << "tmp before: " << tmp << "\n";
    DOUBLE_INTO(tmp, 5);
    std::cout << "tmp after DOUBLE_INTO(tmp, 5) = " << tmp << "\n";
    return 0;
}
