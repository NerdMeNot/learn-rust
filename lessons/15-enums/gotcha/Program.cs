Console.WriteLine(Area(new Triangle(2.0, 3.0)));

static double Area(Shape s) => s switch
{
    Circle(var r) => 3.14 * r * r,
    Rect(var w, var h) => w * h,
};

abstract record Shape;
record Circle(double Radius) : Shape;
record Rect(double Width, double Height) : Shape;
record Triangle(double Base, double Height) : Shape;   // new!
