#include <iostream>
#include <map>
#include <string>
#include <vector>

struct Person {
    std::string name;
    int age;
};

int main() {
    std::vector<Person> people{{"Ada", 36}, {"Linus", 54}};
    int total = 0;
    for (const auto p : people) { // each element is copied, name and all
        total += p.age;
    }
    for (const auto &p : people) { // each element is only looked at
        total += p.age;
    }
    std::map<std::string, int> ages{{"Ada", 36}};
    for (const std::pair<std::string, int> &kv : ages) { // the map holds pair<const string, int>: this copies each entry
        total += kv.second;
    }
    std::cout << "total: " << total << "\n";
    return 0;
}
