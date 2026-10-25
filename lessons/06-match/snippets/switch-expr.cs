static string Classify(int n) => n switch
{
    0 => "zero",
    >= 1 and <= 5 => "few",
    6 or 7 => "some",
    < 0 => "negative",
    > 100 => "big",
    _ => "many",
};

static string Where((int x, int y) p) => p switch
{
    (0, 0) => "origin",
    (> 0, > 0) => "upper right",
    _ => "elsewhere",
};

static string Ends(int[] a) => a switch
{
    [] => "empty",
    [var only] => $"only {only}",
    [var first, .., var last] => $"{first} to {last}",
};

static string Describe(object? o) => o switch
{
    null => "nothing",
    Circle c => $"circle {c.R}",
    Rect { W: > 5 } => "wide rect",
    int and not 0 => "a non-zero int",
    _ => "something else",
};

foreach (var n in new[] { 0, 3, 7, 12, -1, 500 })
{
    Console.WriteLine($"{n} -> {Classify(n)}");
}
Console.WriteLine(Where((0, 0)));
Console.WriteLine(Where((2, 3)));
Console.WriteLine(Ends(new[] { 1, 2, 3, 4 }));
Console.WriteLine(Describe(null));
Console.WriteLine(Describe(new Circle(2)));
Console.WriteLine(Describe(new Rect(9, 1)));
Console.WriteLine(Describe(7));
Console.WriteLine(Describe(0));

record Circle(int R);
record Rect(int W, int H);
