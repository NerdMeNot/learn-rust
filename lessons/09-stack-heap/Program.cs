var a = new PointClass { X = 1, Y = 2 };
var b = a; // a class: b is another name for the same object
b.X = 99;
Console.WriteLine($"class:  a.X = {a.X}, b.X = {b.X}");

var s = new PointStruct { X = 1, Y = 2 };
var t = s; // a struct: t is a copy
t.X = 99;
Console.WriteLine($"struct: s.X = {s.X}, t.X = {t.X}");

class PointClass
{
    public int X, Y;
}

struct PointStruct
{
    public int X, Y;
}
