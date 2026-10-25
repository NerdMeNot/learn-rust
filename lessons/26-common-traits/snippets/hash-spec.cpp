#include <functional>
#include <iostream>
#include <string>
#include <unordered_map>

struct Version {
    unsigned major;
    unsigned minor;
    bool operator==(const Version &) const = default;
};

// Hash: a specialisation of std::hash, kept in step with operator== by hand.
template <>
struct std::hash<Version> {
    size_t operator()(const Version &v) const {
        return std::hash<unsigned>()(v.major) * 31 + std::hash<unsigned>()(v.minor);
    }
};

int main() {
    std::unordered_map<Version, std::string> notes;
    notes[Version{1, 2}] = "stable";
    std::cout << "found: " << notes[Version{1, 2}] << "\n";
}
