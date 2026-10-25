#include <coroutine>
#include <cstdio>
#include <cstdlib>
#include <new>

struct Task {
    struct promise_type {
        // the compiler calls this to get memory for the coroutine's frame
        static void *operator new(std::size_t size) {
            std::printf("frame: operator new(%zu)\n", size);
            return std::malloc(size);
        }
        static void operator delete(void *p) { std::free(p); }
        Task get_return_object() { return Task{std::coroutine_handle<promise_type>::from_promise(*this)}; }
        std::suspend_always initial_suspend() { return {}; }
        std::suspend_always final_suspend() noexcept { return {}; }
        void return_void() {}
        void unhandled_exception() {}
    };
    std::coroutine_handle<promise_type> handle;
};

Task job() {
    char buf[1000]; // used after the suspension, so it lives in the frame
    buf[0] = 'x';
    co_await std::suspend_always{};
    std::printf("buf[0] = %c\n", buf[0]);
}

int main() {
    Task t = job();
    std::printf("the Task you hold is %zu bytes: a handle to the frame\n", sizeof(t));
    t.handle.resume();
    t.handle.resume();
    t.handle.destroy();
}
