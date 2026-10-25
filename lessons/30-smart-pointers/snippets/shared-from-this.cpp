#include <iostream>
#include <memory>

struct Widget : std::enable_shared_from_this<Widget> {
    std::shared_ptr<Widget> share() { return shared_from_this(); }
};

int main() {
    auto owned = std::make_shared<Widget>();
    std::cout << "owners: " << owned->share().use_count() << "\n";

    Widget on_stack; // not owned by any shared_ptr
    try {
        on_stack.share();
    } catch (const std::bad_weak_ptr &e) {
        std::cout << "thrown at run time: bad_weak_ptr\n";
    }
}
