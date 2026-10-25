#include <cstdio>
#include <dirent.h>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <string>

namespace fs = std::filesystem;

// how many write system calls this process has made so far (Linux: the syscw line)
static long write_calls() {
    std::ifstream f("/proc/self/io");
    std::string line;
    while (std::getline(f, line))
        if (line.rfind("syscw:", 0) == 0) return std::stol(line.substr(6));
    return -1;
}

static int open_fds() {
    int n = 0;
    DIR *d = opendir("/proc/self/fd");
    while (readdir(d)) n++;
    closedir(d);
    return n;
}

int main() {
    std::ifstream missing("/no/such/file.txt");
    if (!missing) std::cout << "open failed: checked with if (!f), no exception thrown\n";

    const char *path = "/tmp/lr-fio-cpp.txt";
    {
        std::ofstream w(path);
        w << "x";
    }
    int base = open_fds();
    {
        std::ifstream f(path);
        std::cout << "inside the block: " << open_fds() - base << " more open\n";
    }
    std::cout << "after the block: " << open_fds() - base << " more open\n";
    std::remove(path);

    // an ofstream buffers (a Rust File does not)
    {
        std::ofstream out(path);
        long before = write_calls();
        for (int i = 0; i < 1000; i++) out << "line " << i << "\n";
        long during = write_calls() - before;
        out.close();
        std::cout << "1000 <<: " << during << " write calls before close, " << write_calls() - before << " after\n";
    }
    std::remove(path);

    // the destructor flushes and closes, and cannot report a failure; close() then fail() can.
    // /dev/full refuses every byte, like a full disk.
    std::ofstream full("/dev/full");
    full << "data";
    full.close();
    std::cout << "close() on /dev/full, then fail(): " << full.fail() << "\n";

    // std::filesystem::path is the nearest relative of Path
    std::cout << "path / absolute: " << (fs::path("/data") / "/etc/hosts") << "\n";
    std::cout << "extension() of a.tar.gz: " << fs::path("a.tar.gz").extension()
              << ", of .bashrc: " << fs::path(".bashrc").extension() << "\n";
    try {
        std::cout << fs::file_size("/no/such/file.txt");
    } catch (const fs::filesystem_error &e) {
        std::cout << "file_size throws: " << e.what() << "\n";
    }
}
