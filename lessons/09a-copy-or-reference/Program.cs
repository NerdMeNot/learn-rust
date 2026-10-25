using System;

int[] ages = { 30, 40, 50 };

// A copy: the foreach variable is read-only, so copy it to change it.
foreach (var age in ages)
{
    var copy = age + 1;
    Console.WriteLine($"the copy is now {copy}");
}
Console.WriteLine($"ages after the copy loop:     [{string.Join(", ", ages)}]");

// The elements themselves: assign through the index.
for (int i = 0; i < ages.Length; i++)
{
    ages[i]++;
}
Console.WriteLine($"ages after the in-place loop: [{string.Join(", ", ages)}]");
