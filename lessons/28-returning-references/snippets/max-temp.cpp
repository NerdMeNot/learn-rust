#include <algorithm>
#include <iostream>
#include <string>

int main() {
    // std::max returns a reference to one of its arguments, like `longer` in Rust
    const std::string& m = std::max(std::string("apple pie, a long text"), std::string("banana bread, a long text"));
    (void)m; // reading `m` now would read a destroyed string
    std::cout << "built\n";
}
