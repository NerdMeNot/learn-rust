var p = new Point(3, 4); // a line comment
var items = new List<int> { 1, 2, 3 };
if (items.Count != 3) throw new Exception("items should have 3 elements");
Console.WriteLine($"point: Point {{ x: {p.X}, y: {p.Y} }}");
Console.WriteLine($"items: [{string.Join(", ", items)}]");
Console.WriteLine($"row: [{42,5}] [{42,-5}] [{42:D5}]");
var pi = 3.14159.ToString("F2");
Console.WriteLine($"pi: {pi} {{done}}");

/// <summary>A point on a grid.</summary>
record Point(int X, int Y);
