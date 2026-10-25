#include "counter-cxx/src/counter.h"
#include <cstdio>
#include <stdexcept>
#include "counter-cxx/src/main.rs.h"

Counter::Counter(std::string name) : name_(std::move(name)) {}

Counter::~Counter() {
    std::printf("C++: ~Counter(%s)\n", name_.c_str());
    std::fflush(stdout);
}

void Counter::add(int value) { values_.push_back(value); }

Stats Counter::stats() const {
    int total = 0;
    for (int v : values_) total += v;
    return Stats{values_.size(), total};
}

const std::string &Counter::name() const { return name_; }

int Counter::at(std::size_t index) const {
    if (index >= values_.size()) throw std::out_of_range("index out of range");
    return values_[index];
}

std::unique_ptr<Counter> new_counter(rust::Str name) {
    return std::make_unique<Counter>(std::string(name));
}

// C++ calling back into Rust: `describe` is declared in the bridge and defined in main.rs.
void print_report(const Counter &counter) {
    rust::String text = describe(counter.stats());
    std::printf("C++ asked Rust: %s\n", std::string(text).c_str());
    std::fflush(stdout);
}
