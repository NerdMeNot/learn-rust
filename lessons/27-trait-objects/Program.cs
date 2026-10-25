IShape[] shapes = { new Square(3), new Rect(4, 6) };

int total = 0;
foreach (IShape s in shapes)
{
    Console.WriteLine($"{s.Name()}: {s.Area()}");
    total += s.Area();
}
Console.WriteLine($"total: {total}");

interface IShape
{
    string Name();
    int Area();
}

class Square(int side) : IShape
{
    public string Name() => "square";
    public int Area() => side * side;
}

class Rect(int w, int h) : IShape
{
    public string Name() => "rect";
    public int Area() => w * h;
}
