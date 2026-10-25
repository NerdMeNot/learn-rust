#include <iostream>

struct UserId {
    explicit UserId(int v) : value(v) {}
    int value;
};

struct OrderId {
    explicit OrderId(int v) : value(v) {}
    int value;
};

void load_user(UserId id) {
    std::cout << "user #" << id.value << "\n";
}

int main() {
    OrderId order{7};
    load_user(order);
}
