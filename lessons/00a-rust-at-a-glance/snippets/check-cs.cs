static void Must(bool ok, string what)
{
    if (!ok) throw new Exception(what);
}

static int Add(int a, int b) => a + b;

var x = 5;
x = 6;
const int y = 5;
Must(x == 6 && y == 5, "variables");
Must(Add(2, 3) == 5, "method");

var count = 0;
for (int i = 0; i < 3; i++)
{
    count += i;
}
Must(count == 3, "for");

var words = new List<string> { "a", "b" };
var joined = "";
foreach (var w in words)
{
    joined += w;
}
Must(joined == "ab", "foreach");

var n = 1;
var s = n switch { 1 => "one", _ => "many" };
Must(s == "one", "switch expression");

Func<int, int> inc = v => v + 1;
Must(inc(1) == 2, "lambda");

var name = "Ana";
var age = 30;
Console.WriteLine($"{name} is {age}");

var threw = false;
try
{
    int.Parse("x");
}
catch (FormatException)
{
    threw = true;
}
Must(threw, "Parse throws");
Console.WriteLine("all c# cells compile and hold");
