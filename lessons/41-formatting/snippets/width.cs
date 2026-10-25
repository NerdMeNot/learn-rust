using System;

var id = new Id(42);
Console.WriteLine($"[{id,8}] [{id,-8}]");
Console.WriteLine($"{0.5:F0} {1.5:F0} {2.5:F0}");
Console.WriteLine(1234567.5.ToString("N2"));

record Id(int N)
{
    public override string ToString() => $"#{N}";
}
