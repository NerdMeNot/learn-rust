(int Lo, int Hi) MinMax(int a, int b) => (Math.Min(a, b), Math.Max(a, b));

var r = MinMax(9, 4);
Console.WriteLine($"names: {r.Lo} {r.Hi}, positions: {r.Item1} {r.Item2}");
Console.WriteLine($"equal: {r == (4, 9)}, struct: {r.GetType().IsValueType}");

var (lo, hi) = r;
Console.WriteLine($"deconstructed: {lo} {hi}");

(lo, hi) = (hi, lo);
Console.WriteLine($"swap: {lo} {hi}");

// The usual way .NET returns a second value.
if (int.TryParse("42", out var n))
{
    Console.WriteLine($"TryParse: {n}");
}
