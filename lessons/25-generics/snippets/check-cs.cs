using System.Numerics;

// Backs the C# cells of the table in lesson.mdx. Never shown.
void Must(bool ok, string what)
{
    if (!ok) throw new Exception(what);
}

Must(Bigger(3, 9) == 9, "two bounds");
Must(Make<List<int>>().Count == 0, "new() constraint");
Must(Total([1, 2, 3]) == 6, "INumber");
Must(Total([1.5, 2.5]) == 4.0, "INumber double");
IEnumerable<object> objects = new List<string> { "x" };
Must(objects.Count() == 1, "out T variance");
Must(new List<int> { 1 }.Count == 1, "primitive type argument");
Must(Describe(7) == "7 True 7", "unbounded T");
Must(new Pair<int>(1, 2).Map(n => "#" + n).First == "#1", "generic method");
Must(Array.Empty<string>().Length == 0, "explicit type argument");
Console.WriteLine("ok");

static T Bigger<T>(T a, T b) where T : IComparable<T>, IFormattable => a.CompareTo(b) > 0 ? a : b;

static string Describe<T>(T x) => x?.ToString() + " " + x?.Equals(x) + " " + x?.GetHashCode();

static T Make<T>() where T : new() => new T();

static T Total<T>(T[] xs) where T : INumber<T>
{
    T sum = T.Zero;
    foreach (var x in xs) sum += x;
    return sum;
}

record Pair<T>(T First, T Second)
{
    public Pair<R> Map<R>(Func<T, R> f) => new Pair<R>(f(First), f(Second));
}
