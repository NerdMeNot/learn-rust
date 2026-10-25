#include <condition_variable>
#include <iostream>
#include <mutex>
#include <queue>
#include <thread>
#include <utility>
#include <vector>

template <class T>
class Channel {   // the same small channel as the lesson, cut down
public:
    void send(T v) {
        { std::lock_guard lock(m_); q_.push(std::move(v)); }
        cv_.notify_one();
    }
    T recv() {
        std::unique_lock lock(m_);
        cv_.wait(lock, [&] { return !q_.empty(); });
        T v = std::move(q_.front());
        q_.pop();
        return v;
    }
private:
    std::mutex m_;
    std::condition_variable cv_;
    std::queue<T> q_;
};

void print(const char* label, const std::vector<int>& xs) {
    std::cout << label << " [";
    for (std::size_t i = 0; i < xs.size(); ++i) std::cout << (i ? ", " : "") << xs[i];
    std::cout << "]\n";
}

int main() {
    Channel<std::vector<int>> ch;
    std::vector<int> msg{1, 2};
    std::thread sender([&] {
        ch.send(std::move(msg));
        msg.push_back(3);
    });
    print("received", ch.recv());
    sender.join();
    print("sender's msg is now", msg);
}
