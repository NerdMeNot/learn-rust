#include <chrono>
#include <condition_variable>
#include <iostream>
#include <mutex>
#include <optional>
#include <queue>
#include <stdexcept>
#include <thread>

// The Channel of the lesson's first program, with a timed receive and a size().
template <class T>
class Channel {
public:
    void send(T value) {
        { std::lock_guard lock(m_); q_.push(std::move(value)); }
        cv_.notify_one();
    }
    void close() {
        { std::lock_guard lock(m_); closed_ = true; }
        cv_.notify_all();
    }
    // like recv_timeout: nullopt means closed-and-drained OR timed out; `timed_out` says which
    std::optional<T> recv_for(std::chrono::milliseconds d, bool &timed_out) {
        std::unique_lock lock(m_);
        timed_out = !cv_.wait_for(lock, d, [&] { return !q_.empty() || closed_; });
        if (q_.empty()) return std::nullopt;
        T value = std::move(q_.front());
        q_.pop();
        return value;
    }
    size_t size() { std::lock_guard lock(m_); return q_.size(); }

private:
    std::mutex m_;
    std::condition_variable cv_;
    std::queue<T> q_;
    bool closed_ = false;
};

int main() {
    // 1. A producer that throws before close(): the receiver is never told.
    Channel<int> a;
    std::thread producer([&a] {
        try {
            a.send(1);
            throw std::runtime_error("producer failed");
            a.close();   // never reached
        } catch (const std::exception &) {}
    });
    producer.join();
    bool timed_out = false;
    a.recv_for(std::chrono::milliseconds(200), timed_out);   // gets the 1
    a.recv_for(std::chrono::milliseconds(200), timed_out);
    std::cout << "after the producer threw, recv timed out: " << std::boolalpha << timed_out << "\n";

    // 2. send after close() is accepted, and nothing says the receiver is gone.
    Channel<int> b;
    b.close();
    b.send(2);
    std::cout << "queued after close(): " << b.size() << "\n";
    for (int i = 0; i < 1000; i++) b.send(i);   // no consumer: the queue only grows
    std::cout << "queued with nobody receiving: " << b.size() << "\n";
}
