using System;

object[] things = { new Point(3, 0), new Point(2, 5), (1, 2), "text" };
foreach (var thing in things)
{
    string text = thing switch
    {
        Point(var x, 0) => $"on the x axis at {x}",
        Point { X: > 0 and < 10 } p => $"small point {p}",
        (int a, int b) => $"a pair, sum {a + b}",
        _ => "something else",
    };
    Console.WriteLine(text);
}

record Point(int X, int Y);
