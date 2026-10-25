// 1  a local that changes
var x = 5;
x += 1;
Console.WriteLine($"1  {x}");

// 2  null with a default
string? name = args.Length > 0 ? args[0] : null;
Console.WriteLine($"2  {name ?? "anon"}");

// 3  call through a maybe-missing value
string? word = "ana";
Console.WriteLine($"3  {word?.Length}");

// 4  parse without an exception
string text = "42";
if (int.TryParse(text, out var n))
    Console.WriteLine($"4  {n}");

// 5  string interpolation with width and zero padding
int seven = 7;
Console.WriteLine($"5  [{seven,5}] [{seven:D3}]");

// 6  a LINQ pipeline
var xs = new List<int> { 3, 1, 2 };
var doubled = xs.Where(v => v > 1).Select(v => v * 2).ToList();
Console.WriteLine($"6  {string.Join(",", doubled)}");

// 7  sum and max
Console.WriteLine($"7  {xs.Sum()} {xs.Max()}");

// 8  walk a sorted dictionary
var ages = new SortedDictionary<string, int> { ["bo"] = 31, ["ana"] = 30 };
foreach (var (k, v) in ages)
    Console.WriteLine($"8  {k}={v}");

// 9  look a key up
if (ages.TryGetValue("bo", out var bo))
    Console.WriteLine($"9  {bo}");

// 10  count words
var counts = new Dictionary<string, int>();
foreach (var w in "a b a".Split(' '))
    counts[w] = counts.GetValueOrDefault(w) + 1;
Console.WriteLine($"10 a={counts["a"]} b={counts["b"]}");

// 11  a switch expression
var size = x switch { 1 => "one", _ => "many" };
Console.WriteLine($"11 {size}");

// 12  test the kind of a value and take it apart
Shape sh = x > 5 ? new Circle(2) : new Square(1);
if (sh is Circle c)
    Console.WriteLine($"12 radius {c.R}");
else if (sh is Square sq)
    Console.WriteLine($"12 side {sq.Side}");

// 13  a record: equality and `with`
var p1 = new Point(1, 2);
var p2 = p1 with { X = 3 };
Console.WriteLine($"13 {p1 == new Point(1, 2)} {p2}");

// 14  length in characters
string accent = "héllo";
string emoji = "a😀";
Console.WriteLine($"14 {accent.Length} {emoji.Length}");

// 15  a lock around shared state
var gate = new object();
int hits = 0;
lock (gate) { hits++; }
Console.WriteLine($"15 {hits}");

// 16  a property
var person = new Person { Age = 30 };
person.Age += 1;
Console.WriteLine($"16 {person.Age}");

abstract record Shape;
record Circle(double R) : Shape;
record Square(double Side) : Shape;
record Point(int X, int Y);

class Person
{
    public int Age { get; set; }
}
