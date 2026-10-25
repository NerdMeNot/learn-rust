#pragma once
#include <memory>
#include <string>
#include <vector>
#include "rust/cxx.h"

struct Stats;  // shared with Rust: defined in the generated header

class Counter {
public:
    explicit Counter(std::string name);
    ~Counter();
    void add(int value);
    Stats stats() const;
    const std::string &name() const;
    int at(std::size_t index) const;  // throws std::out_of_range

private:
    std::string name_;
    std::vector<int> values_;
};

std::unique_ptr<Counter> new_counter(rust::Str name);
void print_report(const Counter &counter);
