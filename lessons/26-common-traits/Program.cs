// A record struct writes Equals, GetHashCode and ToString from its properties.
// Ordering is not automatic: IComparable is the one to implement.
var a = new Version(1, 10);
var b = new Version(1, 9);
Console.WriteLine($"display: {a}");
Console.WriteLine($"a == b: {(a == b).ToString().ToLower()}");
Console.WriteLine($"a > b: {(a.CompareTo(b) > 0).ToString().ToLower()}");

var all = new List<Version> { a, b, new Version(0, 0) };
all.Sort();
Console.WriteLine($"sorted: {string.Join(" ", all)}");

var seen = new HashSet<Version>();
foreach (var v in new[] { a, b, new Version(0, 0), a }) seen.Add(v);
Console.WriteLine($"distinct: {seen.Count}");

readonly record struct Version(int Major, int Minor) : IComparable<Version>
{
    public int CompareTo(Version other) =>
        Major != other.Major ? Major.CompareTo(other.Major) : Minor.CompareTo(other.Minor);

    // ToString is the user-facing text (a record's own would be "Version { Major = 1, Minor = 10 }").
    public override string ToString() => $"v{Major}.{Minor}";
}
