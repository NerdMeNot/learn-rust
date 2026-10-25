#include <filesystem>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>

namespace fs = std::filesystem;

int main() {
    fs::path path = fs::temp_directory_path() / "rbc-notes.txt";

    // write a whole file (std::ofstream creates it, or empties it first)
    std::ofstream(path) << "milk\neggs\n";

    // open for appending and add a line. The file closes at the end of the block.
    {
        std::ofstream f(path, std::ios::app);
        f << "bread\n";
    }

    // read the whole file into one string
    std::ifstream in(path);
    std::stringstream buf;
    buf << in.rdbuf();
    std::string all = buf.str();
    std::cout << all.size() << " bytes\n";

    // read line by line
    std::ifstream lines(path);
    std::string line;
    for (int i = 1; std::getline(lines, line); i++) {
        std::cout << i << ": " << line << "\n";
    }

    // a path prints with quotes
    std::cout << "file name: " << path.filename() << "\n";

    // streams only say "it failed". std::filesystem can tell you why.
    std::error_code ec;
    auto size = fs::file_size("/no/such/file.txt", ec);
    if (!ec) {
        std::cout << "found it? " << size << " bytes\n";
    } else {
        std::cout << "error: " << ec.message()
                  << " (not found: " << std::boolalpha << (ec == std::errc::no_such_file_or_directory) << ")\n";
    }

    fs::remove(path);
}
