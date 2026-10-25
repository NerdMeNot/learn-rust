#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <unistd.h>

static void write_file(const char *path, const char *text) {
    std::ofstream(path) << text;
}

static void run(const char *label, const char *cmd) {
    std::printf("%s\n", label);
    std::fflush(stdout);
    std::system(cmd);
}

int main() {
    char dir[] = "/tmp/cppfilesXXXXXX";
    mkdtemp(dir);
    chdir(dir);

    // An unnamed namespace is private to its file, like `static`.
    write_file("lib.cpp", "namespace { int hidden() { return 20; } }\n"
                          "int visible() { return hidden(); }\n");
    write_file("main_bad.cpp", "#include <cstdio>\n"
                               "int hidden();\n"
                               "int main() { std::printf(\"hidden: %d\\n\", hidden()); }\n");
    run("1. another file calls a name from an unnamed namespace",
        "g++ main_bad.cpp lib.cpp -o app 2>&1 | grep -o 'undefined reference.*'");

    // C++20 modules: only what is `export`ed leaves the module.
    write_file("shop.cppm", "export module shop;\n"
                            "int rate() { return 20; }\n"
                            "export int total(int q) { return q * 10 - q * 10 * rate() / 100; }\n");
    write_file("main_ok.cpp", "import shop;\n#include <cstdio>\n"
                              "int main() { std::printf(\"total: %d\\n\", total(12)); }\n");
    write_file("main_bad2.cpp", "import shop;\n#include <cstdio>\n"
                                "int main() { std::printf(\"rate: %d\\n\", rate()); }\n");
    const char *flags = "g++ -std=c++20 -fmodules-ts ";
    char cmd[512];
    std::snprintf(cmd, sizeof cmd, "%s-x c++ -c shop.cppm -o shop.o 2>&1", flags);
    std::system(cmd);
    std::snprintf(cmd, sizeof cmd, "%smain_ok.cpp shop.o -o app 2>&1 && ./app", flags);
    run("2. a C++20 module: call the exported function", cmd);
    std::snprintf(cmd, sizeof cmd, "%smain_bad2.cpp shop.o -o app 2>&1 | grep -o \"error: .*\"", flags);
    run("3. call the function that is not exported", cmd);

    // The linker rule is the same as in C: one definition of a name in the program.
    write_file("one.cpp", "int helper() { return 1; }\nint main() { return helper(); }\n");
    write_file("two.cpp", "int helper() { return 2; }\n");
    run("4. two files define a function with the same name",
        "g++ one.cpp two.cpp -o app 2>&1 | grep -o 'multiple definition of [^;]*'");
}
