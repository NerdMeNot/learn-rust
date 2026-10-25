static int Area(int width, int height = 1) => width * height;

static int Parse(string s) => int.Parse(s) + 1;

static void Check(bool ok, string what)
{
    if (!ok) throw new InvalidOperationException(what);
}

// ++ and --, also inside expressions
var a = new[] { 10, 20, 30 };
var i = 0;
var first = a[i++];
var second = a[++i];
Check(first == 10 && second == 30 && i == 2, "inc");

// the ternary operator
var label = i > 1 ? "big" : "small";
Check(label == "big", "ternary");

// nullable types, ?. and ??
string? name = null;
string? other = "abc";
Check(name is null && name?.Length == null && other?.Length == 3 && (name ?? "anon") == "anon", "null");

// TryParse with out
Check(int.TryParse("12", out var n) && n == 12 && !int.TryParse("x", out _), "tryparse");

// new and this
var p = new Point(1, 2);
Check(p.X == 1 && p.Y == 2, "new");

// overloads and operator overloading
Check(Shapes.Show(1) == "int 1" && Shapes.Show("a") == "text a", "overload");
Check((new Vec(1, 2) + new Vec(3, 4)).X == 4, "operator");

// optional and named arguments
Check(Area(3) == 3 && Area(3, height: 4) == 12 && Area(height: 4, width: 3) == 12, "named");

// stacked case labels share a body, and a body must not fall through
var o = "";
switch (2)
{
    case 1:
    case 2:
        o += "small";
        break;
    default:
        o += "other";
        break;
}
Check(o == "small", "switch");

// exceptions
var trace = "";
try
{
    Parse("x");
    Check(false, "no exception");
}
catch (FormatException)
{
    trace += "caught,";
}
finally
{
    trace += "finally";
}
Check(trace == "caught,finally" && Parse("41") == 42, "catch");

// using disposes at the end of the scope
var log = "";
{
    using var res = new Res(s => log += s);
    log += "body,";
}
Check(log == "body,disposed", "using");

// foreach
var total = 0;
foreach (var x in new[] { 5, 6 }) total += x;
Check(total == 11, "foreach");

// record with
var r1 = new Rec(1, 2);
var r2 = r1 with { X = 5 };
Check(r2.X == 5 && r2.Y == 2, "with");

// inheritance
Animal d = new Dog("Rex");
Check(d.Name == "Rex", "inherit");

// int to long needs no cast
long big = i;
Check(big == 2L, "widen");

Console.WriteLine("csharp: all checks passed");

record Rec(int X, int Y);

class Res : IDisposable
{
    readonly Action<string> log;
    public Res(Action<string> log) { this.log = log; }
    public void Dispose() { log("disposed"); }
}

class Vec
{
    public int X, Y;
    public Vec(int x, int y) { X = x; Y = y; }
    public static Vec operator +(Vec a, Vec b) => new Vec(a.X + b.X, a.Y + b.Y);
}

class Point
{
    public int X, Y;
    public Point(int x, int y)
    {
        this.X = x;
        this.Y = y;
    }
}

class Animal
{
    public string Name;
    public Animal(string name) { Name = name; }
}

class Dog : Animal
{
    public Dog(string name) : base(name) { }
}

static class Shapes
{
    public static string Show(int n) => "int " + n;
    public static string Show(string s) => "text " + s;
}
