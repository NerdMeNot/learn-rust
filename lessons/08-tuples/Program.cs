var (lo, hi) = MinMax([4, 9, 1, 7]);
Console.WriteLine($"{lo} {hi}");

var (_, onlyHi) = MinMax([3, 5]);   // _ ignores a value
Console.WriteLine(onlyHi);

static (int Lo, int Hi) MinMax(int[] xs)
{
    int lo = xs[0], hi = xs[0];
    foreach (var x in xs)
    {
        if (x < lo) lo = x;
        if (x > hi) hi = x;
    }
    return (lo, hi);
}
