#include <iostream>
#include <memory>
#include <string>
#include <thread>
#include <vector>

struct Job {
    std::string name;
    std::shared_ptr<std::vector<std::string>> log;  // copying the pointer is thread-safe; the vector is not locked
};

void run(Job job) {
    job.log->push_back(job.name);
}

int main() {
    auto log = std::make_shared<std::vector<std::string>>();
    Job job{"resize", log};
    std::thread t(run, job);
    t.join();
    std::cout << "log: [\"" << (*log)[0] << "\"]\n";
}
