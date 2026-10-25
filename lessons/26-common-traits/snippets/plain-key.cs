var plain = new Dictionary<Plain, string> { [new Plain(1, 2)] = "stable" };
Console.WriteLine($"plain class: {(plain.TryGetValue(new Plain(1, 2), out var p) ? p : "missing")}");

var half = new Dictionary<HalfDone, string> { [new HalfDone(1, 2)] = "stable" };
Console.WriteLine($"equals only: {(half.TryGetValue(new HalfDone(1, 2), out var h) ? h : "missing")}");
Console.WriteLine($"equals says: {new HalfDone(1, 2).Equals(new HalfDone(1, 2))}");

var rec = new Dictionary<Version, string> { [new Version(1, 2)] = "stable" };
Console.WriteLine($"record: {rec[new Version(1, 2)]}");

class Plain(int major, int minor)
{
    public int Major { get; } = major;
    public int Minor { get; } = minor;
}

// Equals written, GetHashCode forgotten: the C# compiler warns about this one.
class HalfDone(int major, int minor)
{
    public int Major { get; } = major;
    public int Minor { get; } = minor;

    public override bool Equals(object? o) => o is HalfDone h && h.Major == Major && h.Minor == Minor;
}

record Version(int Major, int Minor);
