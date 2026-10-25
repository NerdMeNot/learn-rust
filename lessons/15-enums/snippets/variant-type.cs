Console.WriteLine($"radius: {Radius(new Circle(1.0))}");

// each case is its own type, so a method can ask for exactly one
static double Radius(Circle c) => c.R;

abstract record Shape;
record Circle(double R) : Shape;
record Rect(double W, double H) : Shape;
