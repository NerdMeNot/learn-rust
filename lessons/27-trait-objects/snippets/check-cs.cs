using System;
using System.Collections.Generic;

var shapes = new List<IShape> { new Square(3), new Rect() };
// `is` with a declaration
if (shapes[0] is not Square sq || sq.Side != 3) throw new Exception("is");
if (shapes[1] is Square) throw new Exception("a Rect is not a Square");
// a switch expression over types
Func<object, string> kind = o => o switch
{
    Square s => "square " + s.Side,
    Rect => "rect",
    _ => "other",
};
if (kind(shapes[0]) != "square 3" || kind(shapes[1]) != "rect") throw new Exception("switch");
// a null element
IShape? none = null;
if (none != null) throw new Exception("null");
// an abstract class can hold a field
Base b = new Derived();
if (b.Area() != 7) throw new Exception("abstract");
// a generic constraint
static int Twice<T>(T s) where T : IShape => 2 * s.Area();
if (Twice(new Square(3)) != 18) throw new Exception("generic");
// an interface property (not a field)
INamed named = new Dog();
if (named.Name != "dog") throw new Exception("property");
Console.WriteLine("ok");

interface IShape
{
    int Area();
}

class Square(int side) : IShape
{
    public int Side => side;
    public int Area() => side * side;
}

class Rect : IShape
{
    public int Area() => 24;
}

abstract class Base
{
    protected int id = 7;
    public abstract int Area();
}

class Derived : Base
{
    public override int Area() => id;
}

interface INamed
{
    string Name { get; }
}

class Dog : INamed
{
    public string Name => "dog";
}
