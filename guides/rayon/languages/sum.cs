using System;
using System.Linq;

class Sum
{
    static void Main()
    {
        long total = Enumerable.Range(1, 1_000_000).AsParallel().Select(n => (long)n * n).Sum();
        Console.WriteLine(total);
    }
}
