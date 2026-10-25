#include <coroutine>
#include <iostream>

// C++20 gives the compiler side of a coroutine; the type that says what it does
// when created, resumed and finished is yours (or a library's).
struct Task {
    struct promise_type {
        Task get_return_object() { return Task{std::coroutine_handle<promise_type>::from_promise(*this)}; }
        std::suspend_always initial_suspend() { return {}; } // lazy: stop before the body
        std::suspend_never final_suspend() noexcept { return {}; }
        void return_void() {}
        void unhandled_exception() {}
    };
    std::coroutine_handle<promise_type> handle;
};

Task task() {
    std::cout << "task body runs\n";
    co_return;
}

int main() {
    std::cout << "start\n";
    Task t = task();
    std::cout << "task created\n";
    t.handle.resume();
    std::cout << "end\n";
}
