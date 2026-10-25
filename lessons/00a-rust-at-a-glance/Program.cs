// Greet someone, then add up 1 to 4.
static string Greet(string name) => $"Hello, {name}!";

static int Add(int a, int b) => a + b;

Console.WriteLine(Greet("Ferris"));
var total = 0;
for (var n = 1; n <= 4; n++)
{
    total = Add(total, n);
}
Console.WriteLine($"total: {total}");
