#include <atomic>
#include <iostream>
#include <thread>

int main() {
    std::atomic<bool> saw_stop = false;
    {
        std::jthread t([&](std::stop_token st) {
            while (!st.stop_requested()) std::this_thread::yield();
            saw_stop = true;
        });
        t.request_stop();
    }   // ~jthread joins
    std::cout << "worker saw the stop request: " << std::boolalpha << saw_stop << "\n";
}
