using System;
using System.Collections.Generic;

static T Largest<T>(IReadOnlyList<T> list)
{
    T best = list[0];
    foreach (T item in list)
    {
        if (item > best)
        {
            best = item;
        }
    }
    return best;
}

var numbers = new List<int> { 3, 9, 4 };
var words = new List<string> { "pear", "fig", "apple" };
Console.WriteLine($"largest number: {Largest(numbers)}");
Console.WriteLine($"largest word: {Largest(words)}");
