static long Depth(long n) => n == 0 ? 0 : 1 + Depth(n - 1);

Console.WriteLine($"depth(500): {Depth(500)}");
try
{
    Console.WriteLine($"depth(100_000_000): {Depth(100_000_000)}");
}
catch (Exception)
{
    Console.WriteLine("caught"); // never reached
}
