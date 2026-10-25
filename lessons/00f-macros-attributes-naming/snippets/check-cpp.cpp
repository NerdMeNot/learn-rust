#include <format>
#include <iostream>
#include <string>

static int bad = 0;

static void same(const std::string &got, const std::string &want) {
    if (got != want) {
        std::cout << "got [" << got << "], want [" << want << "]\n";
        bad = 1;
    }
}

int main() {
    same(std::format("{:5}", 42), "   42");
    same(std::format("{:<5}", 42), "42   ");
    same(std::format("{:05}", 42), "00042");
    same(std::format("{:+}", 42), "+42");
    same(std::format("{:.2f}", 3.14159), "3.14");
    same(std::format("{:8.3f}", 3.14159), "   3.142");
    same(std::format("{:x}", 255), "ff");
    same(std::format("{:#x}", 255), "0xff");
    same(std::format("{:>10}", "Ada"), "       Ada");
    same(std::format("{:10}", "Ada"), "Ada       ");
    same(std::format("{:.3}", "Lovelace"), "Lov");
    same(std::format("{:.2}", 3.14159), "3.1");
    same(std::format("{:d}", 5), "5");
    same(std::format("{} {}", 42, "Ada"), "42 Ada");
    same(std::format("{{}}"), "{}");
    if (!bad) std::cout << "ok\n";
    return bad;
}
