using System;

int[] ages = { 30, 40, 50 };
foreach (var age in ages)
{
    age++;
}
Console.WriteLine($"[{string.Join(", ", ages)}]");
