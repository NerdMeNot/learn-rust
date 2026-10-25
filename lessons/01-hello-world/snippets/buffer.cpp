#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <iostream>

// The program under test prints a line, then dies without a clean exit.
int main(int argc, char **argv) {
    if (argc > 1 && std::strcmp(argv[1], "child") == 0) {
        std::cout << "a line, with a newline\n";
        std::abort();
    }
    FILE *p = popen("./main child 2>/dev/null", "r");
    char buf[100] = "";
    size_t n = fread(buf, 1, sizeof buf - 1, p);
    buf[n] = '\0';
    pclose(p);
    std::cout << "stdout is a pipe: read " << n << " bytes: \"" << buf << "\"\n";
    return 0;
}
