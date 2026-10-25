#include <iostream>
#include <memory>
#include <string>
#include <vector>

struct Point {
    int x;
};

static void report(const std::string &kind, bool changed) {
    std::cout << kind << std::string(12 - kind.size(), ' ')
              << (changed ? "shares: the other name sees the change" : "copy:   the other name is untouched") << "\n";
}

int main() {
    int n = 1;
    int m = n;
    m = 99;
    report("int", n == m);

    Point p{1};
    Point q = p;
    q.x = 99;
    report("struct", p.x == q.x);

    std::vector<int> v{1, 2};
    std::vector<int> w = v; // copies every element
    w[0] = 99;
    report("vector", v[0] == w[0]);

    int &r = n; // a reference is another name for n
    r = 99;
    report("reference", n == r);

    Point *pp = &p;
    Point *pq = pp;
    pq->x = 99;
    report("pointer", p.x == pq->x);

    auto sp = std::make_shared<Point>(Point{1});
    auto sq = sp;
    sq->x = 99;
    report("shared_ptr", sp->x == sq->x);

    // Identity: compare addresses
    std::cout << "v.data() == w.data(): " << (v.data() == w.data()) << "  (two buffers)\n";
    std::cout << "&n == &r: " << (&n == &r) << "  (a reference has its target's address)\n";

    // A copy of a struct copies each member. A vector member copies its elements; a pointer member copies the arrow.
    struct Holder {
        std::vector<std::string> names;
        std::shared_ptr<int> count;
    };
    Holder h1{{"ana"}, std::make_shared<int>(1)};
    Holder h2 = h1;
    h2.names[0] = "zed";
    *h2.count = 99;
    std::cout << "after h2 = h1 and changes through h2: h1.names[0] = " << h1.names[0]
              << ", *h1.count = " << *h1.count << "\n";
    return 0;
}
