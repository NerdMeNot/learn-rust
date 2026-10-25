#include <iostream>

// On a function: only this function warns.
[[nodiscard]] int load(int id) {
    return id > 0 ? 0 : -1;
}

// On a type: every function that returns a Status warns.
struct [[nodiscard]] Status {
    int code;
};

Status save(int id) {
    return {id > 0 ? 0 : -1};
}

int main() {
    load(-5);  // the failure code is thrown away
    save(-5);  // so is this one
    std::cout << "done\n";
}
