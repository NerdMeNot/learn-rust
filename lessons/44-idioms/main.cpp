#include <iostream>
#include <stdexcept>
#include <string>

// The constructor is private, so Port::parse is the only way in.
class Port {
public:
    static Port parse(const std::string& text) {
        std::size_t used = 0;
        unsigned long n = 0;
        try {
            n = std::stoul(text, &used);
        } catch (const std::exception&) {
            throw std::invalid_argument("not a number");
        }
        if (used != text.size()) throw std::invalid_argument("not a number");
        if (n < 1 || n > 65535) throw std::invalid_argument(std::to_string(n) + " is outside 1-65535");
        return Port(static_cast<unsigned short>(n));
    }

    unsigned short number() const { return number_; }

private:
    explicit Port(unsigned short number) : number_(number) {}
    unsigned short number_;
};

void listen_on(const Port& port) {
    std::cout << "listening on " << port.number() << "\n";
}

int main() {
    for (const std::string text : {"8080", "0", "http"}) {
        try {
            listen_on(Port::parse(text));
        } catch (const std::invalid_argument& e) {
            std::cout << '"' << text << "\": " << e.what() << "\n";
        }
    }
}
