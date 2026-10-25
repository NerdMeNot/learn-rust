#include <iostream>
#include <string>
#include <unordered_map>

struct Version {
    unsigned major;
    unsigned minor;
};

int main() {
    std::unordered_map<Version, std::string> notes;
    notes[Version{1, 2}] = "stable";
    std::cout << "found: " << notes[Version{1, 2}] << "\n";
}
