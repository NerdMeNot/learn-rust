var shapes = new List<IShape> { new Square(3), new Rect(4, 6) };

int total = 0;
foreach (IShape s in shapes)
{
    total += s.Area();
}
Console.WriteLine($"total: {total}");

interface IShape
{
    int Area();
}

class Square(int side) : IShape
{
    public int Area() => side * side;
}

class Rect(int w, int h) : IShape
{
    public int Area() => w * h;
}
