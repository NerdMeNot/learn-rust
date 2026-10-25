#include <iostream>
#include <mutex>
#include <thread>

int main() {
    std::mutex a, b;
    int moved = 0;

    // The two threads name the mutexes in opposite orders.
    // std::scoped_lock takes both with a deadlock-avoidance algorithm, so this finishes.
    std::thread t1([&] {
        for (int i = 0; i < 1000; i++) {
            std::scoped_lock both(a, b);
            moved++;
        }
    });
    std::thread t2([&] {
        for (int i = 0; i < 1000; i++) {
            std::scoped_lock both(b, a);
            moved++;
        }
    });
    t1.join();
    t2.join();
    std::cout << "moved: " << moved << "\n";
}
