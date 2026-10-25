using System;
using System.Collections.Generic;
using System.IO;

static T Largest<T>(IReadOnlyList<T> list) where T : IComparable<T>
{
    T best = list[0];
    foreach (T item in list)
    {
        if (item.CompareTo(best) > 0) best = item;
    }
    return best;
}

static int Apply(Func<int, int> f, int x) => f(x);

static int? Find(Dictionary<string, int> m, string k) => m.TryGetValue(k, out var v) ? v : null;

static string Read(string path) => File.ReadAllText(path);

static void Bump(ref int x) { x++; }

if (Largest(new List<int> { 3, 9, 4 }) != 9) throw new Exception("largest");
if (Apply(x => x + 1, 1) != 2) throw new Exception("apply");
if (Find(new Dictionary<string, int>(), "a") != null) throw new Exception("find");
try
{
    Read("/nope");
    throw new Exception("read");
}
catch (FileNotFoundException)
{
    // nothing in the signature said this could happen
}
int n = 1;
Bump(ref n);
if (n != 2) throw new Exception("bump");
Console.WriteLine($"int bits: {sizeof(int) * 8}");
Console.WriteLine("csharp rows ok");
