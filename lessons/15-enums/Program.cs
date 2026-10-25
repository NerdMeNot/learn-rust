Shape[] shapes = [new Circle(1.0), new Rect(2.0, 3.0)];
foreach (var s in shapes)
{
    Console.WriteLine(Area(s));
}

static double Area(Shape s) => s switch
{
    Circle(var r) => 3.14 * r * r,
    Rect(var w, var h) => w * h,
    _ => throw new ArgumentException("unknown shape"),
};

abstract record Shape;
record Circle(double Radius) : Shape;
record Rect(double Width, double Height) : Shape;
