#include <iostream>
#include <memory>
#include <string>
#include <vector>

struct Leaky {
    explicit Leaky(std::string n) : name(n) {}
    std::string name;
    std::shared_ptr<Leaky> other;
    ~Leaky() { std::cout << "freed " << name << "\n"; }
};

struct Tree {
    explicit Tree(std::string n) : name(n) {}
    std::string name;
    std::weak_ptr<Tree> parent;
    std::vector<std::shared_ptr<Tree>> children;
    ~Tree() { std::cout << "freed " << name << "\n"; }
};

int main() {
    {
        auto a = std::make_shared<Leaky>("a");
        auto b = std::make_shared<Leaky>("b");
        a->other = b;
        b->other = a;
        std::cout << "owners of a: " << a.use_count() << "\n";
    }
    std::cout << "after the cycle's block: a and b not freed\n";

    {
        auto root = std::make_shared<Tree>("root");
        auto leaf = std::make_shared<Tree>("leaf");
        root->children.push_back(leaf);
        leaf->parent = root; // weak_ptr: does not count
        std::cout << "owners of root: " << root.use_count() << "\n";
    }
    std::cout << "after the tree's block\n";
}
