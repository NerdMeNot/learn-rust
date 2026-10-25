struct Point {
    int x;
    int y;
}

int main(void) {
    struct Point p = {1, 2};
    return p.x + p.y;
}
