Shape[] shapes = { new Circle(1.0), new Square(2.0) };
foreach (var s in shapes)
    Console.WriteLine(s.Describe());

abstract class Shape
{
    public abstract string Name { get; }
    public abstract double Area();
    public string Describe() => $"{Name}: {Area():F2}";
}

class Circle(double r) : Shape
{
    public override string Name => "circle";
    public override double Area() => Math.PI * r * r;
}

class Square(double side) : Shape
{
    public override string Name => "square";
    public override double Area() => side * side;
}
