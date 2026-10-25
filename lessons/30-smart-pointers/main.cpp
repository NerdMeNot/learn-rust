#include <iostream>
#include <memory>
#include <string>
#include <vector>

int main() {
    // shared_ptr counts its owners, as Rc does; unlike Rc, every owner may change the list
    auto log = std::make_shared<std::vector<std::string>>();
    auto other = log;

    log->push_back("from log");
    other->push_back("from other");

    std::cout << "log: ";
    for (size_t i = 0; i < log->size(); i++) {
        std::cout << (i ? ", " : "") << (*log)[i];
    }
    std::cout << "\n";
    std::cout << "same list: " << (log == other ? "true" : "false") << "\n";
}
