using System.Text;

var owned = "héllo";                            // strings never change
ReadOnlySpan<char> view = owned.AsSpan(0, 1);   // a view into it, no copy

var s = new StringBuilder("go");                // the growable kind
s.Append(" + rust");

Console.WriteLine($"{owned} {view} {s}");
Console.WriteLine($"bytes={Encoding.UTF8.GetByteCount(owned)} chars={owned.Length}");
foreach (var c in owned)
{
    Console.Write($"[{c}]");
}
Console.WriteLine();
