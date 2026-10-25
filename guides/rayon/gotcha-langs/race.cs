using System;
using System.Threading.Tasks;

class Race
{
    static void Main()
    {
        long total = 0; // shared by every worker thread
        Parallel.For(1, 1_000_001, n => { total += n; });
        Console.WriteLine($"total: {total} expected: 500000500000");
    }
}
