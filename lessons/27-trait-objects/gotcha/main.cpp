#include <iostream>

class Shape {
public:
    virtual ~Shape() = default;
    virtual unsigned area() const = 0;
};

class Square : public Shape {
public:
    explicit Square(unsigned side) : side_(side) {}
    unsigned area() const override { return side_ * side_; }

private:
    unsigned side_;
};

class Rect : public Shape {
public:
    Rect(unsigned w, unsigned h) : w_(w), h_(h) {}
    unsigned area() const override { return w_ * h_; }

private:
    unsigned w_, h_;
};

int main() {
    Shape shapes[] = {Square(3), Rect(4, 6)};

    unsigned total = 0;
    for (const Shape& s : shapes) total += s.area();
    std::cout << "total: " << total << "\n";
}
