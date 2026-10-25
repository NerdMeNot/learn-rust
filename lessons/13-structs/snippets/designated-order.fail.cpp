struct User {
    const char *name;
    unsigned age;
};

int main() {
    User u{.age = 30, .name = "ana"};
    return u.age == 30 ? 0 : 1;
}
