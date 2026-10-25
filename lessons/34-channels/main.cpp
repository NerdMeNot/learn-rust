#include <condition_variable>
#include <format>
#include <iostream>
#include <mutex>
#include <optional>
#include <queue>
#include <string>
#include <thread>
#include <utility>

// The standard library has no channel, so here is a small one.
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
    std::optional<T> recv() {            // empty optional once closed and drained
        std::unique_lock lock(m_);
        cv_.wait(lock, [&] { return !q_.empty() || closed_; });
        if (q_.empty()) return std::nullopt;
        T value = std::move(q_.front());
        q_.pop();
        return value;
    }

private:
    std::mutex m_;
    std::condition_variable cv_;
    std::queue<T> q_;
    bool closed_ = false;
};

int main() {
    Channel<std::string> ch;

    std::jthread producer([&ch] {
        for (int i = 1; i <= 3; ++i) {
            ch.send(std::format("job {}", i));   // moved into the channel
        }
        ch.close();                              // no more messages
    });

    while (auto msg = ch.recv()) {               // ends when the channel closes
        std::cout << "got " << *msg << "\n";
    }
}
