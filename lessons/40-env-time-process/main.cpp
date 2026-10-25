#include <chrono>
#include <cstdio>
#include <cstdlib>
#include <iostream>
#include <string>
#include <sys/wait.h>
#include <thread>

using namespace std::chrono_literals;

int main(int argc, char *argv[]) {
    (void)argv;
    // argc counts the program itself, so one is subtracted
    std::cout << argc - 1 << " arg(s) after the program name\n";

    // std::getenv returns nullptr when the variable is not set
    const char *p = std::getenv("RBC_PORT");
    std::string port = p ? p : "8080";
    std::cout << "port " << port << "\n";
    if (const char *v = std::getenv("RBC_MISSING")) {
        std::cout << "RBC_MISSING=" << v << "\n";
    } else {
        std::cout << "RBC_MISSING: not set\n";
    }

    // steady_clock never jumps when the system clock changes
    auto start = std::chrono::steady_clock::now();
    std::this_thread::sleep_for(20ms);
    auto took = std::chrono::steady_clock::now() - start;
    std::cout << std::boolalpha << "slept at least 20ms: " << (took >= 20ms) << "\n";

    // std has no process API: POSIX popen runs the command through /bin/sh
    FILE *child = popen("echo 'hello from echo'", "r");
    if (!child) return 1;
    std::string out;
    char buf[128];
    while (std::fgets(buf, sizeof buf, child)) out += buf;
    int status = pclose(child);
    std::cout << out;
    std::cout << "exit ok: " << (WIFEXITED(status) && WEXITSTATUS(status) == 0) << "\n";
}
