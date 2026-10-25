#include <coroutine>

// C++20 ships <coroutine> (the handle and the awaiter types), not a ready-made task type
std::task<int> answer() {
    co_return 42;
}

int main() {
    answer();
}
