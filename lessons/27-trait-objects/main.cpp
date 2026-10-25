#include <iostream>
#include <memory>
#include <string>
#include <vector>

class Shape {
public:
    virtual ~Shape() = default;
    virtual std::string name() const = 0;
    virtual unsigned area() const = 0;
};

class Square : public Shape {
public:
    explicit Square(unsigned side) : side_(side) {}
    std::string name() const override { return "square"; }
    unsigned area() const override { return side_ * side_; }

private:
    unsigned side_;
};

class Rect : public Shape {
public:
    Rect(unsigned w, unsigned h) : w_(w), h_(h) {}
    std::string name() const override { return "rect"; }
    unsigned area() const override { return w_ * h_; }

private:
    unsigned w_, h_;
};

int main() {
    // a vector of objects of different classes holds pointers to them
    std::vector<std::unique_ptr<Shape>> shapes;
    shapes.push_back(std::make_unique<Square>(3));
    shapes.push_back(std::make_unique<Rect>(4, 6));

    unsigned total = 0;
    for (const auto& s : shapes) {
        std::cout << s->name() << ": " << s->area() << "\n";
        total += s->area();
    }
    std::cout << "total: " << total << "\n";
}
