using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

static string Get(int[] a, int i) => i < a.Length ? $"Some({a[i]})" : "None";

int[] prices = [10, 20, 30];
// no `unsafe` keyword needed: the Unsafe class does pointer-style arithmetic on a ref
int second = Unsafe.Add(ref MemoryMarshal.GetArrayDataReference(prices), 1);
Console.WriteLine($"second, read by offset: {second}");
Console.WriteLine($"get(1): {Get(prices, 1)}");
Console.WriteLine($"get(9): {Get(prices, 9)}");
