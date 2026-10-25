#include <iostream>
#include <string>
#include <string_view>

int main() {
    std::string owned = "héllo";                       // owns the text (UTF-8 bytes), can grow
    std::string_view view = std::string_view(owned).substr(0, 1);   // a view into it, no copy

    std::string s = "go";
    s += " + rust";

    std::cout << owned << " " << view << " " << s << "\n";

    // std::string counts bytes. Counting characters means skipping UTF-8 continuation bytes.
    std::size_t chars = 0;
    for (unsigned char b : owned) {
        if ((b & 0xC0) != 0x80) chars++;
    }
    std::cout << "bytes=" << owned.size() << " chars=" << chars << "\n";

    for (std::size_t i = 0; i < owned.size();) {
        std::size_t len = 1;
        while (i + len < owned.size() && (static_cast<unsigned char>(owned[i + len]) & 0xC0) == 0x80) len++;
        std::cout << "[" << owned.substr(i, len) << "]";
        i += len;
    }
    std::cout << "\n";
}
