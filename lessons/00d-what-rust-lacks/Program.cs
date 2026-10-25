// What other languages write with ?:, i++, null and switch, written in C#.
static string Size(int n) => n switch
{
    1 or 2 => "small",
    3 => "medium",
    _ => "large",
};

static int? FirstEven(int[] items)
{
    foreach (var x in items)
    {
        if (x % 2 == 0) return x;
    }
    return null;
}

var total = 0;
for (var i = 0; i < 4; i++)
{
    total += i;
}
var label = total > 5 ? "big" : "small";
Console.WriteLine($"total: {total} ({label})");
Console.WriteLine($"size: {Size(2)} {Size(9)}");
var even = FirstEven(new[] { 3, 5, 8 });
if (even is not null)
{
    Console.WriteLine($"first even: {even}");
}
else
{
    Console.WriteLine("no even number");
}
Console.WriteLine($"none: {FirstEven(new[] { 1, 3 }) ?? -1}");
long wide = (long)total * 1_000_000_000;
Console.WriteLine($"wide: {wide}");
