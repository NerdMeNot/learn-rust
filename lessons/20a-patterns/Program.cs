using System;
using System.Linq;

var (x, y) = new Point(3, -2);
Console.WriteLine($"x = {x}, y = {y}");

foreach (var (c, i) in "abc".Select((c, i) => (c, i)))
{
    Console.WriteLine($"{i}: {c}");
}

if ("mode=fast".Split('=', 2) is [var key, var value])
{
    Console.WriteLine($"{key} -> {value}");
}

// a list pattern: .. stands for the items in the middle
int[] a = { 10, 20, 30, 40 };
if (a is [var first, .., var last])
{
    Console.WriteLine($"first = {first}, last = {last}");
}

record Point(int X, int Y);
