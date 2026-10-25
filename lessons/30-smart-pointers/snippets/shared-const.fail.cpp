#include <memory>

int main() {
    std::shared_ptr<const int> c = std::make_shared<int>(1);
    *c = 2; // a shared_ptr<const T> is read-only, as an Rc<T> is
}
