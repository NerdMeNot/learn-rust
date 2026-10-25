#include <format>
#include <string>

struct P { int n; };
template <> struct std::formatter<P> {
    constexpr auto parse(std::format_parse_context& ctx) { return ctx.begin(); }  // takes no spec
    auto format(const P& p, std::format_context& ctx) const { return std::format_to(ctx.out(), "#{}", p.n); }
};

int main() {
    std::string a = std::format("{:d}", "text");    // wrong type for the letter
    std::string b = std::format("{} {}", 1);        // missing argument
    std::string c = std::format("{:>8}", P{4});     // a spec the formatter does not parse
}
