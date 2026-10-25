#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

static void write_file(const char *path, const char *text) {
    FILE *f = fopen(path, "w");
    fputs(text, f);
    fclose(f);
}

// Runs a shell command; prints only the lines that matter (the link error), or
// the program's output when it builds.
static void run(const char *label, const char *cmd) {
    printf("%s\n", label);
    fflush(stdout);
    system(cmd);
}

int main(void) {
    char dir[] = "/tmp/cfilesXXXXXX";
    mkdtemp(dir);
    chdir(dir);

    write_file("lib.c", "static int hidden(void) { return 20; }\n"
                        "int visible(void) { return hidden(); }\n");
    write_file("main_ok.c", "#include <stdio.h>\n"
                            "int visible(void);\n"
                            "int main(void) { printf(\"visible: %d\\n\", visible()); }\n");
    write_file("main_bad.c", "#include <stdio.h>\n"
                             "int hidden(void);\n"
                             "int main(void) { printf(\"hidden: %d\\n\", hidden()); }\n");

    run("1. another file calls the non-static function",
        "gcc main_ok.c lib.c -o app 2>&1 && ./app");
    run("2. another file calls the static one",
        "gcc main_bad.c lib.c -o app 2>&1 | grep -o 'undefined reference.*'");
    // Without `static`, the name is visible to every file in the program.
    write_file("lib.c", "int hidden(void) { return 20; }\n"
                        "int visible(void) { return hidden(); }\n");
    run("3. the same call, with `static` removed",
        "gcc main_bad.c lib.c -o app 2>&1 && ./app");

    // A header included twice puts its declarations in the file twice.
    write_file("p.h", "struct P { int x; };\n");
    write_file("twice.c", "#include \"p.h\"\n#include \"p.h\"\nint main(void) { return 0; }\n");
    run("4. include a header with a struct twice",
        "gcc twice.c -o app 2>&1 | grep -o 'error: .*'");
    write_file("p.h", "#ifndef P_H\n#define P_H\nstruct P { int x; };\n#endif\n");
    run("5. the same, with a header guard",
        "gcc twice.c -o app 2>&1 && echo '  ok'");

    // Two files that each define a function with the same name.
    write_file("one.c", "int helper(void) { return 1; }\nint main(void) { return helper(); }\n");
    write_file("two.c", "int helper(void) { return 2; }\n");
    run("6. two files define a non-static helper",
        "gcc one.c two.c -o app 2>&1 | grep -o 'multiple definition of [^;]*'");
}
