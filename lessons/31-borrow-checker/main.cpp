#include <iostream>
#include <string>
#include <vector>

struct Item {
    std::string name;
    int count;
};

struct Inventory {
    std::vector<Item> items;
    std::vector<std::string> log;

    void check() {
        for (const Item &item : items) {
            if (item.count == 0) {
                log.push_back("out of stock: " + item.name);
            }
        }
    }
};

int main() {
    Inventory inv{{{"bolt", 0}, {"nut", 12}, {"washer", 0}}, {}};
    inv.check();
    std::cout << "log: [";
    for (size_t i = 0; i < inv.log.size(); i++) {
        std::cout << (i ? ", " : "") << '"' << inv.log[i] << '"';
    }
    std::cout << "]\n";
}
