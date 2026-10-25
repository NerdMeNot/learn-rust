#include <iostream>
#include <memory>

struct Node {
    int value;
    std::unique_ptr<Node> next;
};

int main() {
    std::unique_ptr<Node> head;
    for (int i = 0; i < 1000000; i++) {
        auto node = std::make_unique<Node>();
        node->value = i;
        node->next = std::move(head);
        head = std::move(node);
    }
    std::cout << "built" << std::endl;
    head.reset(); // each ~Node frees the next one: a recursion a million deep
    std::cout << "freed\n";
}
