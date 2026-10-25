#include <stdio.h>
#include <stdlib.h>

// The C below is only a harness: it saves a few #defines and calls, then runs `gcc -E -P`,
// the preprocessor alone, to print what they become.
int main(void) {
    FILE *f = fopen("/tmp/pp-demo.c", "w");
    fputs("#define SQUARE(x) x * x\n"
          "#define SHOW(e) puts(#e)\n"
          "#define LOG(...) printf(__VA_ARGS__)\n"
          "#define MAKE_ADDER(name, n) int name(int x) { return x + (n); }\n"
          "int r = SQUARE(1 + 2);\n"
          "SHOW(a + b);\n"
          "LOG(fmt, a, b);\n"
          "MAKE_ADDER(add_one, 1)\n", f);
    fclose(f);
    fflush(stdout);
    int status = system("gcc -E -P /tmp/pp-demo.c");
    remove("/tmp/pp-demo.c");
    return status == 0 ? 0 : 1;
}
